use anyhow::Context;
use std::path::Path;
use std::process::Command as ProcessCommand;

use crate::release_check::{
    git::{evaluate_head_tag_consistency, load_head_semver_tags},
    metadata::{evaluate_release_metadata, extract_workspace_version_from_toml},
    public_docs::find_public_competitor_content,
    ReleaseCheckItem, ReleaseCheckReport,
};

pub(crate) fn run_release_check(
    repo_root: &Path,
    tag: Option<&str>,
    run_tests: bool,
) -> anyhow::Result<ReleaseCheckReport> {
    let cargo_path = repo_root.join("Cargo.toml");
    let changelog_path = repo_root.join("CHANGELOG.md");

    let cargo_toml = std::fs::read_to_string(&cargo_path)
        .with_context(|| format!("read {}", cargo_path.display()))?;
    let changelog_text = std::fs::read_to_string(&changelog_path)
        .with_context(|| format!("read {}", changelog_path.display()))?;

    let default_tag = extract_workspace_version_from_toml(&cargo_toml)
        .map(|version| format!("v{version}"))
        .unwrap_or_else(|| "v0.0.0".to_string());
    let resolved_tag = tag.map(|value| value.to_string()).unwrap_or(default_tag);

    let mut report = evaluate_release_metadata(&cargo_toml, &changelog_text, &resolved_tag);

    for (id, rel_path) in [
        ("workflow.release", ".github/workflows/release.yml"),
        (
            "workflow.release_dry_run",
            ".github/workflows/release-dry-run.yml",
        ),
        ("script.package_release", "scripts/package_release.py"),
    ] {
        let full = repo_root.join(rel_path);
        let exists = full.exists();
        report.checks.push(ReleaseCheckItem {
            id: id.to_string(),
            ok: exists,
            message: if exists {
                format!("{rel_path} exists")
            } else {
                format!("{rel_path} is missing")
            },
        });
    }

    match find_public_competitor_content(repo_root) {
        Ok(hits) => report.checks.push(ReleaseCheckItem {
            id: "docs.public_competitor_content".to_string(),
            ok: hits.is_empty(),
            message: if hits.is_empty() {
                "public docs do not contain competitor-analysis terms".to_string()
            } else {
                format!(
                    "remove competitor-analysis references from public docs: {}",
                    hits.join("; ")
                )
            },
        }),
        Err(err) => report.checks.push(ReleaseCheckItem {
            id: "docs.public_competitor_content".to_string(),
            ok: false,
            message: format!("failed to scan public docs: {err}"),
        }),
    }

    match load_head_semver_tags(repo_root) {
        Ok(tags) => report
            .checks
            .push(evaluate_head_tag_consistency(&resolved_tag, &tags)),
        Err(err) => report.checks.push(ReleaseCheckItem {
            id: "git.head_tag_consistency".to_string(),
            ok: false,
            message: format!("failed to inspect HEAD semver tags: {err}"),
        }),
    }

    if run_tests {
        let status = ProcessCommand::new("cargo")
            .arg("test")
            .arg("--workspace")
            .arg("--locked")
            .current_dir(repo_root)
            .status()
            .context("run cargo test --workspace --locked")?;
        report.checks.push(ReleaseCheckItem {
            id: "preflight.tests".to_string(),
            ok: status.success(),
            message: format!("cargo test exit status: {status}"),
        });
    } else {
        report.checks.push(ReleaseCheckItem {
            id: "preflight.tests".to_string(),
            ok: true,
            message: "skipped (pass --run-tests to enable)".to_string(),
        });
    }

    report.ok = report.checks.iter().all(|check| check.ok);
    Ok(report)
}

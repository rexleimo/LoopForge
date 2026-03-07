use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ReleaseCheckItem {
    pub(crate) id: String,
    pub(crate) ok: bool,
    pub(crate) message: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ReleaseCheckReport {
    pub(crate) ok: bool,
    pub(crate) tag: String,
    pub(crate) checks: Vec<ReleaseCheckItem>,
}

fn parse_release_tag_version(tag: &str) -> Option<String> {
    let tag = tag.trim();
    let version = tag.strip_prefix('v')?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    if parts
        .iter()
        .all(|p| !p.is_empty() && p.chars().all(|ch| ch.is_ascii_digit()))
    {
        Some(version.to_string())
    } else {
        None
    }
}

fn extract_workspace_version_from_toml(cargo_toml: &str) -> Option<String> {
    let value: toml::Value = toml::from_str(cargo_toml).ok()?;
    value
        .get("workspace")?
        .get("package")?
        .get("version")?
        .as_str()
        .map(|s| s.to_string())
}

fn changelog_has_release_section(changelog_text: &str, version: &str) -> bool {
    let target = format!("## [{version}]");
    changelog_text
        .lines()
        .any(|line| line.trim_start().starts_with(&target))
}

const PUBLIC_DOCS_FORBIDDEN_TERMS: &[&str] = &["openfang", "openclaw"];

fn collect_public_text_files(root: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root).with_context(|| format!("read_dir {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_public_text_files(&path, out)?;
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if matches!(ext, "md" | "markdown" | "yml" | "yaml") {
            out.push(path);
        }
    }
    Ok(())
}

fn find_public_competitor_content(repo_root: &Path) -> anyhow::Result<Vec<String>> {
    let mut files = Vec::new();
    let mkdocs = repo_root.join("mkdocs.yml");
    if mkdocs.exists() {
        files.push(mkdocs);
    }
    collect_public_text_files(&repo_root.join("docs-site"), &mut files)?;

    let mut hits = Vec::new();
    for path in files {
        let raw =
            std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let lower = raw.to_ascii_lowercase();
        for term in PUBLIC_DOCS_FORBIDDEN_TERMS {
            if lower.contains(term) {
                let display = path.strip_prefix(repo_root).unwrap_or(&path).display();
                hits.push(format!("{} ({term})", display));
            }
        }
    }
    hits.sort();
    hits.dedup();
    Ok(hits)
}

fn parse_semver_tags(raw: &str) -> Vec<String> {
    let mut tags: Vec<String> = raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| parse_release_tag_version(line).is_some())
        .map(|line| line.to_string())
        .collect();
    tags.sort();
    tags.dedup();
    tags
}

fn load_head_semver_tags(repo_root: &Path) -> anyhow::Result<Vec<String>> {
    let output = ProcessCommand::new("git")
        .args(["tag", "--points-at", "HEAD"])
        .current_dir(repo_root)
        .output()
        .context("run git tag --points-at HEAD")?;
    if !output.status.success() {
        anyhow::bail!(
            "git tag --points-at HEAD failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(parse_semver_tags(&String::from_utf8_lossy(&output.stdout)))
}

fn evaluate_head_tag_consistency(requested_tag: &str, head_tags: &[String]) -> ReleaseCheckItem {
    let requested_valid = parse_release_tag_version(requested_tag).is_some();
    let (ok, message) = if !requested_valid {
        (
            false,
            format!("cannot compare HEAD tags because requested tag `{requested_tag}` is invalid"),
        )
    } else if head_tags.is_empty() {
        (
            true,
            "no semver release tag on HEAD yet (pre-tag check)".to_string(),
        )
    } else if head_tags.len() == 1 && head_tags[0] == requested_tag {
        (
            true,
            format!("HEAD semver tag `{}` matches requested tag", head_tags[0]),
        )
    } else if head_tags.len() == 1 {
        (
            false,
            format!(
                "HEAD semver tag `{}` does not match requested tag `{requested_tag}`",
                head_tags[0]
            ),
        )
    } else {
        (
            false,
            format!(
                "multiple semver tags point at HEAD: {}; requested `{requested_tag}`",
                head_tags.join(", ")
            ),
        )
    };

    ReleaseCheckItem {
        id: "git.head_tag_consistency".to_string(),
        ok,
        message,
    }
}

fn evaluate_release_metadata(
    cargo_toml: &str,
    changelog_text: &str,
    tag: &str,
) -> ReleaseCheckReport {
    let mut checks = Vec::new();

    let tag_version = parse_release_tag_version(tag);
    checks.push(ReleaseCheckItem {
        id: "tag.format".to_string(),
        ok: tag_version.is_some(),
        message: if tag_version.is_some() {
            format!("tag `{tag}` matches vX.Y.Z")
        } else {
            format!("tag `{tag}` is invalid; expected vX.Y.Z")
        },
    });

    let cargo_version = extract_workspace_version_from_toml(cargo_toml);
    checks.push(ReleaseCheckItem {
        id: "cargo.workspace_version".to_string(),
        ok: cargo_version.is_some(),
        message: match cargo_version.as_deref() {
            Some(v) => format!("workspace version `{v}`"),
            None => "failed to parse [workspace.package].version".to_string(),
        },
    });

    let versions_match = match (tag_version.as_deref(), cargo_version.as_deref()) {
        (Some(tag_v), Some(cargo_v)) => tag_v == cargo_v,
        _ => false,
    };
    checks.push(ReleaseCheckItem {
        id: "cargo.matches_tag".to_string(),
        ok: versions_match,
        message: match (tag_version.as_deref(), cargo_version.as_deref()) {
            (Some(tag_v), Some(cargo_v)) => {
                if tag_v == cargo_v {
                    format!("tag version `{tag_v}` matches Cargo.toml")
                } else {
                    format!("tag version `{tag_v}` does not match Cargo.toml `{cargo_v}`")
                }
            }
            _ => "cannot compare tag and Cargo.toml versions".to_string(),
        },
    });

    let changelog_ok = tag_version
        .as_deref()
        .map(|v| changelog_has_release_section(changelog_text, v))
        .unwrap_or(false);
    checks.push(ReleaseCheckItem {
        id: "changelog.section".to_string(),
        ok: changelog_ok,
        message: match tag_version.as_deref() {
            Some(v) if changelog_ok => format!("found changelog section [{v}]"),
            Some(v) => format!("missing changelog section [{v}]"),
            None => "cannot verify changelog without valid tag".to_string(),
        },
    });

    let ok = checks.iter().all(|c| c.ok);
    ReleaseCheckReport {
        ok,
        tag: tag.to_string(),
        checks,
    }
}

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
        .map(|v| format!("v{v}"))
        .unwrap_or_else(|| "v0.0.0".to_string());
    let resolved_tag = tag.map(|s| s.to_string()).unwrap_or(default_tag);

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

    report.ok = report.checks.iter().all(|c| c.ok);
    Ok(report)
}

pub(crate) fn format_release_check_report(report: &ReleaseCheckReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Release check for {}\n\n", report.tag));
    for check in &report.checks {
        let prefix = if check.ok { "OK  " } else { "ERR " };
        out.push_str(&format!("{prefix} {}: {}\n", check.id, check.message));
    }
    out.push_str(&format!(
        "\nSummary: {}\n",
        if report.ok { "PASS" } else { "FAIL" }
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as ProcessCommand;
    use tempfile::tempdir;

    #[test]
    fn release_report_module_renders_summary() {
        let report = ReleaseCheckReport {
            ok: true,
            tag: "v1.2.3".to_string(),
            checks: vec![ReleaseCheckItem {
                id: "demo".to_string(),
                ok: true,
                message: "fine".to_string(),
            }],
        };

        let out = format_release_check_report(&report);
        assert!(out.contains("Summary: PASS"), "{out}");
    }

    #[test]
    fn release_metadata_check_passes_when_versions_match() {
        let cargo = r#"
[workspace]
members = []

[workspace.package]
version = "0.1.0"
edition = "2021"
"#;
        let changelog = "# Changelog

## [0.1.0] - 2026-03-04
";
        let report = evaluate_release_metadata(cargo, changelog, "v0.1.0");
        assert!(report.ok, "expected release metadata ok, got: {report:?}");
    }

    #[test]
    fn release_metadata_check_fails_when_changelog_missing_section() {
        let cargo = r#"
[workspace]
members = []

[workspace.package]
version = "0.1.0"
edition = "2021"
"#;
        let changelog = "# Changelog

## [Unreleased]
";
        let report = evaluate_release_metadata(cargo, changelog, "v0.1.0");
        assert!(
            !report.ok,
            "expected release metadata fail, got: {report:?}"
        );
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "changelog.section" && !c.ok),
            "expected changelog.section failure, got: {report:?}"
        );
    }

    #[test]
    fn release_check_flags_public_competitor_content() {
        let tmp = tempdir().unwrap();
        std::fs::write(
            tmp.path().join("Cargo.toml"),
            r#"[workspace.package]
version = "1.1.0"
"#,
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("CHANGELOG.md"),
            r#"# Changelog

## [1.1.0] - 2026-03-07
"#,
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("mkdocs.yml"),
            "site_name: LoopForge
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join("docs-site/blog")).unwrap();
        std::fs::write(
            tmp.path().join("docs-site/blog/index.md"),
            "OpenClaw should not appear in public docs.
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join(".github/workflows")).unwrap();
        std::fs::write(
            tmp.path().join(".github/workflows/release.yml"),
            "name: release
",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join(".github/workflows/release-dry-run.yml"),
            "name: release-dry-run
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join("scripts")).unwrap();
        std::fs::write(
            tmp.path().join("scripts/package_release.py"),
            "print('ok')
",
        )
        .unwrap();

        let report = run_release_check(tmp.path(), Some("v1.1.0"), false).unwrap();
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "docs.public_competitor_content" && !c.ok),
            "expected public competitor content failure, got: {report:?}"
        );
    }

    #[test]
    fn release_check_flags_mismatched_head_tag() {
        let tmp = tempdir().unwrap();
        std::fs::write(
            tmp.path().join("Cargo.toml"),
            r#"[workspace.package]
version = "1.1.0"
"#,
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("CHANGELOG.md"),
            r#"# Changelog

## [1.1.0] - 2026-03-07
"#,
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("mkdocs.yml"),
            "site_name: LoopForge
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join("docs-site")).unwrap();
        std::fs::write(
            tmp.path().join("docs-site/index.md"),
            "# LoopForge
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join(".github/workflows")).unwrap();
        std::fs::write(
            tmp.path().join(".github/workflows/release.yml"),
            "name: release
",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join(".github/workflows/release-dry-run.yml"),
            "name: release-dry-run
",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join("scripts")).unwrap();
        std::fs::write(
            tmp.path().join("scripts/package_release.py"),
            "print('ok')
",
        )
        .unwrap();

        assert!(ProcessCommand::new("git")
            .arg("init")
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());
        assert!(ProcessCommand::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());
        assert!(ProcessCommand::new("git")
            .args(["config", "user.name", "LoopForge Test"])
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());
        assert!(ProcessCommand::new("git")
            .args(["add", "."])
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());
        assert!(ProcessCommand::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());
        assert!(ProcessCommand::new("git")
            .args(["tag", "v1.1.1"])
            .current_dir(tmp.path())
            .status()
            .unwrap()
            .success());

        let report = run_release_check(tmp.path(), Some("v1.1.0"), false).unwrap();
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.id == "git.head_tag_consistency" && !c.ok),
            "expected mismatched head tag failure, got: {report:?}"
        );
    }
}

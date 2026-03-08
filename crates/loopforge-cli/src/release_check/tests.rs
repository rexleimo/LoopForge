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
    let report = metadata::evaluate_release_metadata(cargo, changelog, "v0.1.0");
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
    let report = metadata::evaluate_release_metadata(cargo, changelog, "v0.1.0");
    assert!(
        !report.ok,
        "expected release metadata fail, got: {report:?}"
    );
    assert!(
        report
            .checks
            .iter()
            .any(|check| check.id == "changelog.section" && !check.ok),
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
    std::fs::write(tmp.path().join("mkdocs.yml"), "site_name: LoopForge\n").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs-site/blog")).unwrap();
    std::fs::write(
        tmp.path().join("docs-site/blog/index.md"),
        "OpenClaw should not appear in public docs.\n",
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join(".github/workflows")).unwrap();
    std::fs::write(
        tmp.path().join(".github/workflows/release.yml"),
        "name: release\n",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join(".github/workflows/release-dry-run.yml"),
        "name: release-dry-run\n",
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("scripts")).unwrap();
    std::fs::write(
        tmp.path().join("scripts/package_release.py"),
        "print('ok')\n",
    )
    .unwrap();

    let report = run_release_check(tmp.path(), Some("v1.1.0"), false).unwrap();
    assert!(
        report
            .checks
            .iter()
            .any(|check| check.id == "docs.public_competitor_content" && !check.ok),
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
    std::fs::write(tmp.path().join("mkdocs.yml"), "site_name: LoopForge\n").unwrap();
    std::fs::create_dir_all(tmp.path().join("docs-site")).unwrap();
    std::fs::write(tmp.path().join("docs-site/index.md"), "# LoopForge\n").unwrap();
    std::fs::create_dir_all(tmp.path().join(".github/workflows")).unwrap();
    std::fs::write(
        tmp.path().join(".github/workflows/release.yml"),
        "name: release\n",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join(".github/workflows/release-dry-run.yml"),
        "name: release-dry-run\n",
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("scripts")).unwrap();
    std::fs::write(
        tmp.path().join("scripts/package_release.py"),
        "print('ok')\n",
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
            .any(|check| check.id == "git.head_tag_consistency" && !check.ok),
        "expected mismatched head tag failure, got: {report:?}"
    );
}

use anyhow::Context;
use std::path::Path;
use std::process::Command as ProcessCommand;

use crate::release_check::{metadata::parse_release_tag_version, ReleaseCheckItem};

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

pub(super) fn load_head_semver_tags(repo_root: &Path) -> anyhow::Result<Vec<String>> {
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

pub(super) fn evaluate_head_tag_consistency(
    requested_tag: &str,
    head_tags: &[String],
) -> ReleaseCheckItem {
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

use crate::release_check::{ReleaseCheckItem, ReleaseCheckReport};

pub(super) fn parse_release_tag_version(tag: &str) -> Option<String> {
    let tag = tag.trim();
    let version = tag.strip_prefix('v')?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    if parts
        .iter()
        .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
    {
        Some(version.to_string())
    } else {
        None
    }
}

pub(super) fn extract_workspace_version_from_toml(cargo_toml: &str) -> Option<String> {
    let value: toml::Value = toml::from_str(cargo_toml).ok()?;
    value
        .get("workspace")?
        .get("package")?
        .get("version")?
        .as_str()
        .map(|version| version.to_string())
}

fn changelog_has_release_section(changelog_text: &str, version: &str) -> bool {
    let target = format!("## [{version}]");
    changelog_text
        .lines()
        .any(|line| line.trim_start().starts_with(&target))
}

pub(crate) fn evaluate_release_metadata(
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
            Some(version) => format!("workspace version `{version}`"),
            None => "failed to parse [workspace.package].version".to_string(),
        },
    });

    let versions_match = match (tag_version.as_deref(), cargo_version.as_deref()) {
        (Some(tag_version), Some(cargo_version)) => tag_version == cargo_version,
        _ => false,
    };
    checks.push(ReleaseCheckItem {
        id: "cargo.matches_tag".to_string(),
        ok: versions_match,
        message: match (tag_version.as_deref(), cargo_version.as_deref()) {
            (Some(tag_version), Some(cargo_version)) => {
                if tag_version == cargo_version {
                    format!("tag version `{tag_version}` matches Cargo.toml")
                } else {
                    format!(
                        "tag version `{tag_version}` does not match Cargo.toml `{cargo_version}`"
                    )
                }
            }
            _ => "cannot compare tag and Cargo.toml versions".to_string(),
        },
    });

    let changelog_ok = tag_version
        .as_deref()
        .map(|version| changelog_has_release_section(changelog_text, version))
        .unwrap_or(false);
    checks.push(ReleaseCheckItem {
        id: "changelog.section".to_string(),
        ok: changelog_ok,
        message: match tag_version.as_deref() {
            Some(version) if changelog_ok => format!("found changelog section [{version}]"),
            Some(version) => format!("missing changelog section [{version}]"),
            None => "cannot verify changelog without valid tag".to_string(),
        },
    });

    let ok = checks.iter().all(|check| check.ok);
    ReleaseCheckReport {
        ok,
        tag: tag.to_string(),
        checks,
    }
}

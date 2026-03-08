use std::path::Path;

use rexos_skills::manifest::parse_manifest;

use super::discovery::{discover_workspace_skills, home_skills_root};
use super::{SkillsDoctorIssue, SkillsDoctorLevel, SkillsDoctorReport};

pub(crate) fn doctor(workspace_root: &Path) -> anyhow::Result<SkillsDoctorReport> {
    let discovered = discover_workspace_skills(workspace_root)?;
    let home_skills = home_skills_root()?;

    let mut issues = Vec::new();
    if discovered.is_empty() {
        issues.push(SkillsDoctorIssue {
            level: SkillsDoctorLevel::Warn,
            id: "skills.none_found".to_string(),
            message: "no skills discovered in workspace or ~/.codex/skills".to_string(),
            path: None,
        });
    }
    for root in [home_skills, workspace_root.join(".loopforge/skills")] {
        inspect_root(&root, &mut issues)?;
    }

    for skill in discovered.values() {
        let entry_path = skill.root_dir.join(skill.manifest.entry.trim());
        if !entry_path.is_file() {
            issues.push(SkillsDoctorIssue {
                level: SkillsDoctorLevel::Error,
                id: "entry.missing".to_string(),
                message: format!(
                    "skill `{}` entry file missing: {}",
                    skill.name,
                    entry_path.display()
                ),
                path: Some(entry_path.display().to_string()),
            });
        }
    }

    let ok = !issues
        .iter()
        .any(|issue| matches!(issue.level, SkillsDoctorLevel::Error));

    Ok(SkillsDoctorReport {
        ok,
        discovered_count: discovered.len(),
        issues,
    })
}

fn inspect_root(root: &Path, issues: &mut Vec<SkillsDoctorIssue>) -> anyhow::Result<()> {
    if !root.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let skill_dir = entry.path();
        let manifest_path = skill_dir.join("skill.toml");
        if !manifest_path.is_file() {
            continue;
        }

        let raw = match std::fs::read_to_string(&manifest_path) {
            Ok(raw) => raw,
            Err(error) => {
                issues.push(SkillsDoctorIssue {
                    level: SkillsDoctorLevel::Error,
                    id: "manifest.read_failed".to_string(),
                    message: format!("failed to read {}: {error}", manifest_path.display()),
                    path: Some(manifest_path.display().to_string()),
                });
                continue;
            }
        };

        if let Err(error) = parse_manifest(&raw) {
            issues.push(SkillsDoctorIssue {
                level: SkillsDoctorLevel::Error,
                id: "manifest.parse_failed".to_string(),
                message: format!("failed to parse {}: {error}", manifest_path.display()),
                path: Some(manifest_path.display().to_string()),
            });
        }
    }

    Ok(())
}

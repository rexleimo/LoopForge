use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use serde::Serialize;

use rexos::paths::RexosPaths;
use rexos_skills::loader::{discover_skills, DiscoveredSkill, SkillSource};
use rexos_skills::manifest::parse_manifest;

#[derive(Debug, Clone, Serialize)]
pub struct SkillListItem {
    pub name: String,
    pub version: String,
    pub source: String,
    pub root_dir: String,
    pub entry_path: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillsDoctorLevel {
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillsDoctorIssue {
    pub level: SkillsDoctorLevel,
    pub id: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillsDoctorReport {
    pub ok: bool,
    pub discovered_count: usize,
    pub issues: Vec<SkillsDoctorIssue>,
}

pub fn discover_workspace_skills(
    workspace_root: &Path,
) -> anyhow::Result<BTreeMap<String, DiscoveredSkill>> {
    let home_skills = home_skills_root()?;
    discover_skills(workspace_root, &home_skills)
}

pub fn list_skills(workspace_root: &Path) -> anyhow::Result<Vec<SkillListItem>> {
    let discovered = discover_workspace_skills(workspace_root)?;
    let mut out = Vec::with_capacity(discovered.len());
    for (_name, skill) in discovered {
        out.push(skill_to_item(&skill));
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn find_skill(workspace_root: &Path, name: &str) -> anyhow::Result<DiscoveredSkill> {
    let discovered = discover_workspace_skills(workspace_root)?;
    if let Some(skill) = discovered.get(name) {
        return Ok(skill.clone());
    }

    let mut names = discovered.keys().cloned().collect::<Vec<_>>();
    names.sort();
    bail!(
        "skill `{}` not found (available: {})",
        name,
        names.join(", ")
    )
}

pub fn read_skill_entry(skill: &DiscoveredSkill) -> anyhow::Result<String> {
    let entry_path = skill.root_dir.join(skill.manifest.entry.trim());
    let raw = std::fs::read_to_string(&entry_path)
        .with_context(|| format!("read skill entry: {}", entry_path.display()))?;

    let max_chars = 24_000usize;
    let content = if raw.chars().count() > max_chars {
        let head = raw.chars().take(12_000).collect::<String>();
        let tail = raw.chars().rev().take(12_000).collect::<String>();
        format!(
            "{head}\n\n[... truncated {} chars ...]\n\n{}",
            raw.chars().count() - 24_000,
            tail.chars().rev().collect::<String>()
        )
    } else {
        raw
    };
    Ok(content)
}

pub fn permission_tools(permissions: &[String]) -> Vec<String> {
    let mut tools = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for raw in permissions {
        let p = raw.trim().to_ascii_lowercase();
        if p.is_empty() {
            continue;
        }
        if p == "readonly" {
            for t in ["fs_read", "fs_list", "web_search", "web_fetch"] {
                if seen.insert(t.to_string()) {
                    tools.push(t.to_string());
                }
            }
            continue;
        }

        if let Some(tool) = p.strip_prefix("tool:") {
            let tool = tool.trim();
            if !tool.is_empty() && seen.insert(tool.to_string()) {
                tools.push(tool.to_string());
            }
        }
    }

    tools
}

pub fn source_name(source: SkillSource) -> &'static str {
    match source {
        SkillSource::Home => "home",
        SkillSource::Workspace => "workspace",
    }
}

pub fn doctor(workspace_root: &Path) -> anyhow::Result<SkillsDoctorReport> {
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
        .any(|i| matches!(i.level, SkillsDoctorLevel::Error));

    Ok(SkillsDoctorReport {
        ok,
        discovered_count: discovered.len(),
        issues,
    })
}

fn home_skills_root() -> anyhow::Result<PathBuf> {
    let paths = RexosPaths::discover()?;
    let home_dir = paths
        .base_dir
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| anyhow::anyhow!("failed to resolve home directory"))?;
    Ok(RexosPaths::codex_home_skills_dir(&home_dir))
}

fn skill_to_item(skill: &DiscoveredSkill) -> SkillListItem {
    let entry_path = skill.root_dir.join(skill.manifest.entry.trim());
    SkillListItem {
        name: skill.name.clone(),
        version: skill.manifest.version.to_string(),
        source: source_name(skill.source).to_string(),
        root_dir: skill.root_dir.display().to_string(),
        entry_path: entry_path.display().to_string(),
        permissions: skill.manifest.permissions.clone(),
    }
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
            Err(e) => {
                issues.push(SkillsDoctorIssue {
                    level: SkillsDoctorLevel::Error,
                    id: "manifest.read_failed".to_string(),
                    message: format!("failed to read {}: {e}", manifest_path.display()),
                    path: Some(manifest_path.display().to_string()),
                });
                continue;
            }
        };

        if let Err(e) = parse_manifest(&raw) {
            issues.push(SkillsDoctorIssue {
                level: SkillsDoctorLevel::Error,
                id: "manifest.parse_failed".to_string(),
                message: format!("failed to parse {}: {e}", manifest_path.display()),
                path: Some(manifest_path.display().to_string()),
            });
        }
    }

    Ok(())
}

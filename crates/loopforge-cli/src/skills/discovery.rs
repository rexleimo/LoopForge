use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use rexos::paths::RexosPaths;
use rexos_skills::loader::{discover_skills, DiscoveredSkill, SkillSource};

use super::SkillListItem;

pub(crate) fn discover_workspace_skills(
    workspace_root: &Path,
) -> anyhow::Result<BTreeMap<String, DiscoveredSkill>> {
    let home_skills = home_skills_root()?;
    discover_skills(workspace_root, &home_skills)
}

pub(crate) fn list_skills(workspace_root: &Path) -> anyhow::Result<Vec<SkillListItem>> {
    let discovered = discover_workspace_skills(workspace_root)?;
    let mut out = Vec::with_capacity(discovered.len());
    for (_name, skill) in discovered {
        out.push(skill_to_item(&skill));
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub(crate) fn find_skill(workspace_root: &Path, name: &str) -> anyhow::Result<DiscoveredSkill> {
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

pub(crate) fn read_skill_entry(skill: &DiscoveredSkill) -> anyhow::Result<String> {
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

pub(crate) fn source_name(source: SkillSource) -> &'static str {
    match source {
        SkillSource::Home => "home",
        SkillSource::Workspace => "workspace",
    }
}

pub(super) fn home_skills_root() -> anyhow::Result<PathBuf> {
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

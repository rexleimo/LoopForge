use anyhow::Context;

use crate::patch::apply_hunks_to_text;
use crate::Toolset;

pub(super) fn write_new_file(tools: &Toolset, path: &str, content: String) -> anyhow::Result<()> {
    let dest = tools.resolve_workspace_path_for_write(path)?;
    super::parent::ensure_parent_dir(&dest)?;
    std::fs::write(&dest, content).with_context(|| format!("write {}", dest.display()))?;
    Ok(())
}

pub(super) fn rewrite_file(
    tools: &Toolset,
    path: &str,
    hunks: Vec<crate::patch::PatchHunk>,
) -> anyhow::Result<()> {
    let dest = tools.resolve_workspace_path_for_write(path)?;
    let before =
        std::fs::read_to_string(&dest).with_context(|| format!("read {}", dest.display()))?;
    let after = apply_hunks_to_text(&before, &hunks).context("apply hunks")?;
    std::fs::write(&dest, after).with_context(|| format!("write {}", dest.display()))?;
    Ok(())
}

pub(super) fn remove_file(tools: &Toolset, path: &str) -> anyhow::Result<()> {
    let dest = tools.resolve_workspace_path(path)?;
    std::fs::remove_file(&dest).with_context(|| format!("delete {}", dest.display()))?;
    Ok(())
}

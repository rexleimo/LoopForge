mod parent;
#[cfg(test)]
mod tests;
mod write;

use crate::Toolset;

pub(super) fn write_new_file(tools: &Toolset, path: &str, content: String) -> anyhow::Result<()> {
    write::write_new_file(tools, path, content)
}

pub(super) fn rewrite_file(
    tools: &Toolset,
    path: &str,
    hunks: Vec<crate::patch::PatchHunk>,
) -> anyhow::Result<()> {
    write::rewrite_file(tools, path, hunks)
}

pub(super) fn remove_file(tools: &Toolset, path: &str) -> anyhow::Result<()> {
    write::remove_file(tools, path)
}

#[cfg(test)]
pub(super) fn ensure_parent_dir(dest: &std::path::Path) -> anyhow::Result<()> {
    parent::ensure_parent_dir(dest)
}

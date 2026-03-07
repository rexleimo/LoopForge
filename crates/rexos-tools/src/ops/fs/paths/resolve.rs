use std::path::{Path, PathBuf};

use anyhow::bail;

pub(crate) fn resolve_workspace_path_for_write(
    workspace_root: &Path,
    rel: PathBuf,
) -> anyhow::Result<PathBuf> {
    let candidate = workspace_root.join(&rel);
    if candidate.exists() {
        let file_type = std::fs::symlink_metadata(&candidate)?.file_type();
        if file_type.is_symlink() {
            bail!("path is a symlink");
        }
    }
    Ok(candidate)
}

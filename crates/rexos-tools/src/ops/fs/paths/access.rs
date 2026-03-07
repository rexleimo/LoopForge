use std::path::PathBuf;

use crate::Toolset;

impl Toolset {
    pub(crate) fn resolve_workspace_path(&self, user_path: &str) -> anyhow::Result<PathBuf> {
        let rel = super::validate_relative_path(user_path)?;
        let candidate = self.workspace_root.join(&rel);
        self.ensure_no_symlink_escape(&rel)?;
        Ok(candidate)
    }

    pub(crate) fn resolve_workspace_path_for_write(
        &self,
        user_path: &str,
    ) -> anyhow::Result<PathBuf> {
        let rel = super::validate_relative_path(user_path)?;
        self.ensure_no_symlink_escape(&rel)?;
        super::resolve::resolve_workspace_path_for_write(&self.workspace_root, rel)
    }
}

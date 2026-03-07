use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) fn fs_write(&self, user_path: &str, content: &str) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path_for_write(user_path)?;
        if let Some(parent) = super::parent_dir(&path) {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create dirs {}", parent.display()))?;
        }

        std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
        Ok(super::write_success_output().to_string())
    }
}

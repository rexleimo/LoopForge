use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) fn fs_read(&self, user_path: &str) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path(user_path)?;

        let meta = std::fs::metadata(&path).with_context(|| format!("stat {}", path.display()))?;
        if meta.len() > super::shared::FS_READ_MAX_BYTES {
            bail!("file too large: {} bytes", meta.len());
        }

        std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))
    }
}

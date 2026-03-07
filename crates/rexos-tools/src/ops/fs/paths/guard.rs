use std::path::Path;

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) fn ensure_no_symlink_escape(&self, rel: &Path) -> anyhow::Result<()> {
        let mut cur = self.workspace_root.clone();
        for comp in rel.components() {
            if let std::path::Component::Normal(seg) = comp {
                cur.push(seg);
                if cur.exists() {
                    let ft = std::fs::symlink_metadata(&cur)
                        .with_context(|| format!("stat {}", cur.display()))?
                        .file_type();
                    if ft.is_symlink() {
                        bail!("symlinks are not allowed in workspace paths");
                    }
                }
            }
        }
        Ok(())
    }
}

use std::path::{Path, PathBuf};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct RexosPaths {
    pub base_dir: PathBuf,
}

impl RexosPaths {
    pub fn discover() -> anyhow::Result<Self> {
        let home_dir = dirs::home_dir().context("could not resolve home directory")?;
        Ok(Self {
            base_dir: home_dir.join(".rexos"),
        })
    }

    pub fn config_path(&self) -> PathBuf {
        self.base_dir.join("config.toml")
    }

    pub fn db_path(&self) -> PathBuf {
        self.base_dir.join("rexos.db")
    }

    pub fn ensure_dirs(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("create base dir: {}", self.base_dir.display()))?;
        Ok(())
    }

    pub fn is_inside_base(&self, candidate: &Path) -> bool {
        candidate.starts_with(&self.base_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_inside_base_checks_prefix() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().to_path_buf(),
        };

        assert!(paths.is_inside_base(&paths.base_dir.join("a/b/c")));
        assert!(!paths.is_inside_base(Path::new("/tmp/not-rexos")));
    }
}


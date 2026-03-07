use std::path::PathBuf;

use anyhow::{bail, Context};

use crate::{BROWSER_BRIDGE_PATH, BROWSER_BRIDGE_SCRIPT};

pub(super) fn browser_bridge_script_path() -> anyhow::Result<PathBuf> {
    if let Some(path) = env_bridge_script_path()? {
        return Ok(path);
    }

    if let Some(path) = BROWSER_BRIDGE_PATH.get() {
        return Ok(path.clone());
    }

    let dir = std::env::temp_dir().join("rexos");
    std::fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    let path = dir.join("browser_bridge.py");
    std::fs::write(&path, BROWSER_BRIDGE_SCRIPT)
        .with_context(|| format!("write {}", path.display()))?;
    let _ = BROWSER_BRIDGE_PATH.set(path.clone());
    Ok(path)
}

pub(super) fn env_bridge_script_path() -> anyhow::Result<Option<PathBuf>> {
    let Ok(value) = std::env::var("LOOPFORGE_BROWSER_BRIDGE_PATH") else {
        return Ok(None);
    };

    let path = PathBuf::from(value);
    if path.exists() {
        return Ok(Some(path));
    }

    bail!(
        "LOOPFORGE_BROWSER_BRIDGE_PATH does not exist: {}",
        path.display()
    )
}

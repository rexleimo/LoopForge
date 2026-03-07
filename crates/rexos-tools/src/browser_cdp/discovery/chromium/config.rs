use std::path::PathBuf;

use anyhow::bail;

pub(super) fn configured_browser_path(var_name: &str) -> anyhow::Result<Option<PathBuf>> {
    let Ok(path) = std::env::var(var_name) else {
        return Ok(None);
    };

    let path = PathBuf::from(path.trim());
    if path.exists() {
        return Ok(Some(path));
    }

    if var_name == "LOOPFORGE_BROWSER_CHROME_PATH" {
        bail!(
            "LOOPFORGE_BROWSER_CHROME_PATH does not exist: {}",
            path.display()
        );
    }

    Ok(None)
}

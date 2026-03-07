use anyhow::bail;

use super::candidates::chromium_candidates;
use super::names::browser_binary_names;
use super::path::find_in_path;

pub(super) fn find_chromium() -> anyhow::Result<std::path::PathBuf> {
    if let Some(path) = super::config::configured_browser_path("LOOPFORGE_BROWSER_CHROME_PATH")? {
        return Ok(path);
    }
    if let Some(path) = super::config::configured_browser_path("CHROME_PATH")? {
        return Ok(path);
    }

    if let Some(path) = first_existing_candidate() {
        return Ok(path);
    }

    for name in browser_binary_names() {
        if let Some(path) = find_in_path(name) {
            return Ok(path);
        }
    }

    bail!(
        "could not find Chrome/Chromium. Install Chrome/Chromium or set LOOPFORGE_BROWSER_CHROME_PATH."
    )
}

pub(super) fn first_existing_candidate() -> Option<std::path::PathBuf> {
    chromium_candidates()
        .into_iter()
        .find(|candidate| candidate.exists())
}

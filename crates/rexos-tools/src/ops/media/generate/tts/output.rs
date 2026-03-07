use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

use crate::Toolset;

pub(super) fn wav_output_path(
    toolset: &Toolset,
    path: Option<&str>,
) -> anyhow::Result<(String, PathBuf)> {
    let relative_path = path.unwrap_or(".loopforge/audio/tts.wav").to_string();
    let output_path = toolset.resolve_workspace_path_for_write(&relative_path)?;
    if output_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        != "wav"
    {
        bail!("text_to_speech currently only supports .wav output paths");
    }
    Ok((relative_path, output_path))
}

pub(super) fn create_parent_dirs(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create dirs {}", parent.display()))?;
    }
    Ok(())
}

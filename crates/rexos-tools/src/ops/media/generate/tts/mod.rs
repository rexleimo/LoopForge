use anyhow::{bail, Context};

mod output;
mod wave;

use crate::Toolset;

impl Toolset {
    pub(crate) fn text_to_speech(&self, text: &str, path: Option<&str>) -> anyhow::Result<String> {
        if text.trim().is_empty() {
            bail!("text is empty");
        }

        let (relative_path, output_path) = output::wav_output_path(self, path)?;
        output::create_parent_dirs(&output_path)?;
        let bytes = wave::placeholder_tts_wave();

        std::fs::write(&output_path, &bytes)
            .with_context(|| format!("write {}", output_path.display()))?;

        Ok(serde_json::json!({
            "path": relative_path,
            "format": "wav",
            "bytes": bytes.len(),
            "note": "MVP: generates a short WAV tone (placeholder for real TTS).",
        })
        .to_string())
    }
}

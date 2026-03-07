use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) fn media_transcribe(&self, user_path: &str) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path(user_path)?;
        let meta = std::fs::metadata(&path).with_context(|| format!("stat {}", path.display()))?;
        if meta.len() > 2_000_000 {
            bail!("transcript too large: {} bytes", meta.len());
        }

        let ext = path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        if !super::shared::is_supported_transcript_ext(&ext) {
            bail!("media_transcribe currently supports text transcripts (.txt/.md/.srt/.vtt)");
        }

        let raw =
            std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let text = super::shared::trimmed_transcript_text(&raw);

        Ok(serde_json::json!({
            "path": user_path,
            "text": text,
        })
        .to_string())
    }
}

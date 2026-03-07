use anyhow::{bail, Context};

use crate::Toolset;

fn media_kind(ext: &str) -> &'static str {
    match ext {
        "wav" | "mp3" | "flac" | "ogg" | "m4a" | "aac" | "opus" => "audio",
        "mp4" | "mov" | "mkv" | "webm" | "avi" => "video",
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" => "image",
        "txt" | "md" | "srt" | "vtt" => "text",
        _ => "unknown",
    }
}

impl Toolset {
    pub(crate) fn media_describe(&self, user_path: &str) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path(user_path)?;
        let meta = std::fs::metadata(&path).with_context(|| format!("stat {}", path.display()))?;
        if meta.len() > 200_000_000 {
            bail!("media too large: {} bytes", meta.len());
        }

        let ext = path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        Ok(serde_json::json!({
            "path": user_path,
            "bytes": meta.len(),
            "kind": media_kind(&ext),
            "ext": if ext.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(ext) },
        })
        .to_string())
    }
}

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) fn image_analyze(&self, user_path: &str) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path(user_path)?;
        let meta = std::fs::metadata(&path).with_context(|| format!("stat {}", path.display()))?;
        if meta.len() > 10_000_000 {
            bail!("image too large: {} bytes", meta.len());
        }

        let bytes = std::fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let Some((format, width, height)) =
            super::formats::detect_image_format_and_dimensions(&bytes)
        else {
            bail!("unsupported image format (expected png/jpeg/gif)");
        };

        Ok(serde_json::json!({
            "path": user_path,
            "format": format,
            "width": width,
            "height": height,
            "bytes": bytes.len(),
        })
        .to_string())
    }
}

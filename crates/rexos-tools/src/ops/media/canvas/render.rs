use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) fn canvas_present(&self, html: &str, title: Option<&str>) -> anyhow::Result<String> {
        let title = title
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .unwrap_or("Canvas");

        let sanitized = super::sanitize::sanitize_canvas_html(html, 512 * 1024)?;
        let canvas_id = uuid::Uuid::new_v4().to_string();
        let rel = format!("output/canvas_{canvas_id}.html");
        let out_path = self.resolve_workspace_path_for_write(&rel)?;
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create dirs {}", parent.display()))?;
        }

        let safe_title = super::super::escape_xml_text(title);
        let full = format!(
            "<!DOCTYPE html>\n<html>\n<head><meta charset=\"utf-8\"><title>{safe_title}</title></head>\n<body>\n{sanitized}\n</body>\n</html>\n"
        );

        std::fs::write(&out_path, &full)
            .with_context(|| format!("write {}", out_path.display()))?;

        Ok(serde_json::json!({
            "canvas_id": canvas_id,
            "title": title,
            "saved_to": rel,
            "size_bytes": full.len(),
        })
        .to_string())
    }
}

#[path = "image/svg.rs"]
mod svg;
#[cfg(test)]
#[path = "image/tests.rs"]
mod tests;

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) fn image_generate(&self, prompt: &str, user_path: &str) -> anyhow::Result<String> {
        if prompt.trim().is_empty() {
            bail!("prompt is empty");
        }

        let output_path = self.resolve_workspace_path_for_write(user_path)?;
        if !svg::is_svg_output_path(&output_path) {
            bail!("only svg output is supported for now (use a .svg path)");
        }

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create dirs {}", parent.display()))?;
        }

        let svg = svg::placeholder_svg(prompt);
        std::fs::write(&output_path, svg)
            .with_context(|| format!("write {}", output_path.display()))?;

        Ok(serde_json::json!({
            "path": user_path,
            "format": "svg",
        })
        .to_string())
    }
}

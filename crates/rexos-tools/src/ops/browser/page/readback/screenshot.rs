use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_screenshot(&self, path: Option<&str>) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let data = session.screenshot().await?;
        super::super::super::ensure_output_url_allowed(session, &data).await?;

        let bytes = super::shared::decode_screenshot_bytes(&data)?;
        let relative_path = path.unwrap_or(super::shared::DEFAULT_SCREENSHOT_PATH);
        let output_path = self.resolve_workspace_path_for_write(relative_path)?;
        super::shared::write_screenshot_file(&output_path, &bytes)?;

        Ok(super::shared::screenshot_payload(
            relative_path,
            data.get("url").cloned(),
        ))
    }
}

use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_read_page(&self) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let out = session.read_page().await?;
        super::super::super::ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }
}

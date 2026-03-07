use anyhow::Context;

use crate::ops::browser::ensure_output_url_allowed;
use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_click(&self, selector: &str) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let out = session.click(selector).await?;
        ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }

    pub(crate) async fn browser_type(&self, selector: &str, text: &str) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let out = session.type_text(selector, text).await?;
        Ok(out.to_string())
    }

    pub(crate) async fn browser_press_key(
        &self,
        selector: Option<&str>,
        key: &str,
    ) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let out = session.press_key(selector, key).await?;
        ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }
}

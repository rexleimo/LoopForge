use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_wait(
        &self,
        selector: &str,
        timeout_ms: Option<u64>,
    ) -> anyhow::Result<String> {
        super::validate::validate_browser_wait_selector(selector)?;

        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context("browser session not started; call browser_navigate first")?;
        let out = session.wait_for(Some(selector), None, timeout_ms).await?;
        super::super::super::ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }

    pub(crate) async fn browser_wait_for(
        &self,
        selector: Option<&str>,
        text: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> anyhow::Result<String> {
        super::validate::validate_browser_wait_for_inputs(selector, text)?;

        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context("browser session not started; call browser_navigate first")?;
        let out = session.wait_for(selector, text, timeout_ms).await?;
        super::super::super::ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }
}

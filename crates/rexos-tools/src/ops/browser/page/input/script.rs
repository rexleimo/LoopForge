use anyhow::Context;

use crate::ops::browser::ensure_output_url_allowed;
use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_run_js(&self, expression: &str) -> anyhow::Result<String> {
        super::validate_browser_expression(expression)?;

        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context(super::shared::BROWSER_SESSION_REQUIRED)?;
        let out = session.run_js(expression).await?;
        ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }
}

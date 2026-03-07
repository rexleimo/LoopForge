use crate::browser_runtime::{browser_backend_default, browser_headless_default};
use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_navigate(
        &self,
        url: &str,
        _timeout_ms: Option<u64>,
        allow_private: bool,
        headless: Option<bool>,
    ) -> anyhow::Result<String> {
        let url = super::super::validate::validated_browser_url(url, allow_private).await?;
        let backend = browser_backend_default();

        let mut guard = self.browser.lock().await;
        super::super::validate::ensure_session_compatible(&guard, backend, headless)?;

        if guard.is_none() {
            let headless = super::resolved_headless(headless, browser_headless_default());
            *guard =
                Some(super::launch::spawn_session(self, backend, headless, allow_private).await?);
        }

        let session = guard.as_mut().expect("set above");
        session.set_allow_private(allow_private);
        let out = session.navigate(url.as_str()).await?;
        super::super::super::super::ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }
}

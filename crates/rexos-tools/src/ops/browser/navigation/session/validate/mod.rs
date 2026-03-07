mod session;
mod url;

pub(super) async fn validated_browser_url(
    url: &str,
    allow_private: bool,
) -> anyhow::Result<reqwest::Url> {
    url::validated_browser_url(url, allow_private).await
}

pub(super) fn ensure_session_compatible(
    session: &Option<crate::browser_runtime::BrowserSession>,
    backend: crate::browser_runtime::BrowserBackend,
    headless: Option<bool>,
) -> anyhow::Result<()> {
    session::ensure_session_compatible(session, backend, headless)
}

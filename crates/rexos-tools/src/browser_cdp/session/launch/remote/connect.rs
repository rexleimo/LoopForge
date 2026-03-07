use super::super::super::CdpBrowserSession;
use super::super::shared::{connect_page_session, session_from_cdp};

impl CdpBrowserSession {
    pub async fn connect_or_launch(
        http: reqwest::Client,
        headless: bool,
        allow_private: bool,
    ) -> anyhow::Result<Self> {
        if let Some(base_http) = super::config::configured_remote_base()? {
            return Self::connect_remote(http, &base_http, headless, allow_private).await;
        }

        Self::launch_local(http, headless, allow_private).await
    }

    async fn connect_remote(
        http: reqwest::Client,
        base_http: &str,
        headless: bool,
        allow_private: bool,
    ) -> anyhow::Result<Self> {
        let base = super::config::parse_and_validate_remote_base(base_http)?;
        let cdp = connect_page_session(&http, &base).await?;

        Ok(session_from_cdp(headless, allow_private, None, None, cdp))
    }
}

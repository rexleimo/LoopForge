use crate::browser_cdp::CdpBrowserSession;
use crate::browser_runtime::{BrowserBackend, BrowserSession, PlaywrightBrowserSession};
use crate::Toolset;

pub(super) async fn spawn_session(
    tools: &Toolset,
    backend: BrowserBackend,
    headless: bool,
    allow_private: bool,
) -> anyhow::Result<BrowserSession> {
    match backend {
        BrowserBackend::Cdp => {
            let session =
                CdpBrowserSession::connect_or_launch(tools.http.clone(), headless, allow_private)
                    .await?;
            Ok(BrowserSession::Cdp(session))
        }
        BrowserBackend::Playwright => Ok(BrowserSession::Playwright(
            PlaywrightBrowserSession::spawn(headless, allow_private).await?,
        )),
    }
}

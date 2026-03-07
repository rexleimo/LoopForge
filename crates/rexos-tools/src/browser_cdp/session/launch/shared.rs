use super::super::{CdpBrowserSession, CdpConnection};

pub(super) async fn connect_page_session(
    http: &reqwest::Client,
    base: &reqwest::Url,
) -> anyhow::Result<CdpConnection> {
    let page_ws = crate::browser_cdp::discovery::find_or_create_page_ws(http, base).await?;
    let cdp = CdpConnection::connect(&page_ws).await?;

    let _ = cdp.send("Page.enable", serde_json::json!({})).await;
    let _ = cdp.send("Runtime.enable", serde_json::json!({})).await;

    Ok(cdp)
}

pub(super) fn session_from_cdp(
    headless: bool,
    allow_private: bool,
    process: Option<tokio::process::Child>,
    user_data_dir: Option<std::path::PathBuf>,
    cdp: CdpConnection,
) -> CdpBrowserSession {
    CdpBrowserSession {
        headless,
        allow_private,
        process,
        user_data_dir,
        cdp,
    }
}

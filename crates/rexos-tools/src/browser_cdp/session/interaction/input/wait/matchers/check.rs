use crate::browser_cdp::session::CdpBrowserSession;

pub(super) async fn selector_found(session: &CdpBrowserSession, script: &str) -> bool {
    session
        .cdp
        .run_js(script)
        .await
        .ok()
        .map(|value| value.as_str() == Some("found"))
        .unwrap_or(false)
}

pub(super) async fn text_found(session: &CdpBrowserSession, script: &str) -> bool {
    session
        .cdp
        .run_js(script)
        .await
        .ok()
        .map(|value| value.as_str() == Some("found"))
        .unwrap_or(false)
}

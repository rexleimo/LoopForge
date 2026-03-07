mod payload;
mod send;

use crate::browser_cdp::session::helpers::KeyEvent;
use crate::browser_cdp::session::CdpBrowserSession;

#[cfg(test)]
pub(super) fn key_event_payload(
    event_type: &str,
    key: &str,
    code: &str,
    vkey: i32,
) -> serde_json::Value {
    payload::key_event_payload(event_type, key, code, vkey)
}

pub(super) async fn dispatch_known_key(
    session: &CdpBrowserSession,
    event: &KeyEvent,
) -> anyhow::Result<()> {
    send::dispatch_known_key(session, event).await
}

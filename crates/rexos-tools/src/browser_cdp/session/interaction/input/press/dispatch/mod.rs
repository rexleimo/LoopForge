mod fallback;
mod known;
#[cfg(test)]
mod tests;

use crate::browser_cdp::session::helpers::KeyEvent;
use crate::browser_cdp::session::CdpBrowserSession;

#[cfg(test)]
fn key_event_payload(event_type: &str, key: &str, code: &str, vkey: i32) -> serde_json::Value {
    known::key_event_payload(event_type, key, code, vkey)
}

fn fallback_key_script(key: &str) -> String {
    fallback::fallback_key_script(key)
}

pub(super) async fn dispatch_known_key(
    session: &CdpBrowserSession,
    event: &KeyEvent,
) -> anyhow::Result<()> {
    known::dispatch_known_key(session, event).await
}

pub(super) async fn dispatch_fallback_key(session: &CdpBrowserSession, key: &str) {
    fallback::dispatch_fallback_key(session, &fallback_key_script(key)).await
}

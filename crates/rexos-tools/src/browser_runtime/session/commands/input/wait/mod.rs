mod payload;
#[cfg(test)]
mod tests;

use serde_json::Value;

use super::super::super::BrowserSession;

impl BrowserSession {
    pub(crate) async fn press_key(
        &mut self,
        selector: Option<&str>,
        key: &str,
    ) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(session) => session.press_key(selector, key).await,
            Self::Playwright(session) => {
                session
                    .send(payload::press_key_request(selector, key))
                    .await
            }
        }
    }

    pub(crate) async fn wait_for(
        &mut self,
        selector: Option<&str>,
        text: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(session) => session.wait_for(selector, text, timeout_ms).await,
            Self::Playwright(session) => {
                session
                    .send(payload::wait_for_request(selector, text, timeout_ms))
                    .await
            }
        }
    }
}

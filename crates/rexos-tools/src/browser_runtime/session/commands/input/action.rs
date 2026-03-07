use serde_json::Value;

use super::super::super::BrowserSession;

impl BrowserSession {
    pub(crate) async fn click(&mut self, selector: &str) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(session) => session.click(selector).await,
            Self::Playwright(session) => {
                session
                    .send(serde_json::json!({
                        "action": "Click",
                        "selector": selector,
                    }))
                    .await
            }
        }
    }

    pub(crate) async fn type_text(&mut self, selector: &str, text: &str) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(session) => session.type_text(selector, text).await,
            Self::Playwright(session) => {
                session
                    .send(serde_json::json!({
                        "action": "Type",
                        "selector": selector,
                        "text": text,
                    }))
                    .await
            }
        }
    }
}

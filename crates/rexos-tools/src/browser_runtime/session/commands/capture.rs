use serde_json::Value;

use super::super::BrowserSession;

impl BrowserSession {
    pub(crate) async fn read_page(&mut self) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(s) => s.read_page().await,
            Self::Playwright(s) => s.send(serde_json::json!({ "action": "ReadPage" })).await,
        }
    }

    pub(crate) async fn screenshot(&mut self) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(s) => s.screenshot().await,
            Self::Playwright(s) => s.send(serde_json::json!({ "action": "Screenshot" })).await,
        }
    }

    pub(crate) async fn close(&mut self) {
        match self {
            Self::Cdp(s) => {
                s.close().await;
            }
            Self::Playwright(s) => {
                let _ = s.send(serde_json::json!({ "action": "Close" })).await;
                s.kill().await;
            }
        }
    }
}

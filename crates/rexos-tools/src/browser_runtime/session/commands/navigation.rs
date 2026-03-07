#[path = "navigation/payload.rs"]
mod payload;
#[cfg(test)]
#[path = "navigation/tests.rs"]
mod tests;

use serde_json::Value;

use super::super::BrowserSession;
use super::helpers::add_headless_flag;

impl BrowserSession {
    pub(crate) async fn navigate(&mut self, url: &str) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(s) => s
                .navigate(url)
                .await
                .map(|v| add_headless_flag(v, s.headless)),
            Self::Playwright(s) => s
                .send(payload::navigate_request(url))
                .await
                .map(|v| add_headless_flag(v, s.headless)),
        }
    }

    pub(crate) async fn back(&mut self) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(s) => s.back().await,
            Self::Playwright(s) => s.send(payload::back_request()).await,
        }
    }

    pub(crate) async fn scroll(&mut self, direction: &str, amount: i64) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(s) => s.scroll(direction, amount).await,
            Self::Playwright(s) => s.send(payload::scroll_request(direction, amount)).await,
        }
    }
}

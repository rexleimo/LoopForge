use anyhow::Context;
use serde_json::Value;

use crate::browser_cdp::session::helpers::{page_info, wait_for_load};
use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn navigate(&self, url: &str) -> anyhow::Result<Value> {
        self.cdp
            .send("Page.navigate", serde_json::json!({ "url": url }))
            .await
            .context("Page.navigate")?;

        wait_for_load(&self.cdp).await;
        page_info(&self.cdp).await
    }
}

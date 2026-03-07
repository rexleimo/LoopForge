use std::time::Duration;

use serde_json::Value;

use crate::browser_cdp::session::helpers::{page_info, wait_for_load};
use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn back(&self) -> anyhow::Result<Value> {
        let _ = self.cdp.run_js("history.back(); null").await;
        tokio::time::sleep(Duration::from_millis(250)).await;
        wait_for_load(&self.cdp).await;
        page_info(&self.cdp).await
    }
}

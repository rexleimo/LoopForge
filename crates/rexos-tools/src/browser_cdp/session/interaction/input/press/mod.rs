mod dispatch;
mod focus;
mod result;

use std::time::Duration;

use serde_json::Value;

use self::dispatch::{dispatch_fallback_key, dispatch_known_key};
use self::focus::focus_selector_if_present;
use self::result::key_result;
use super::super::super::helpers::{key_event_fields, wait_for_load};
use super::super::super::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn press_key(&self, selector: Option<&str>, key: &str) -> anyhow::Result<Value> {
        focus_selector_if_present(self, selector).await;

        if let Some(event) = key_event_fields(key) {
            dispatch_known_key(self, &event).await?;
        } else {
            dispatch_fallback_key(self, key).await;
        }

        tokio::time::sleep(Duration::from_millis(250)).await;
        wait_for_load(&self.cdp).await;
        key_result(self, selector, key).await
    }
}

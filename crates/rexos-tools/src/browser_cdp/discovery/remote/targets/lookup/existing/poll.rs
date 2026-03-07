use std::time::Duration;

use anyhow::{bail, Context};
use serde_json::Value;

use super::page::page_target_ws;

pub(super) async fn find_existing_page_ws(
    http: &reqwest::Client,
    base: &reqwest::Url,
) -> anyhow::Result<String> {
    let list_url = base.join("/json/list").context("join /json/list")?;
    for attempt in retry_attempts() {
        if attempt > 0 {
            tokio::time::sleep(Duration::from_millis(300)).await;
        }

        let Some(targets) = fetch_targets(http, list_url.clone()).await else {
            continue;
        };
        if let Some(ws) = page_target_ws(&targets) {
            return Ok(ws);
        }
    }

    bail!("no page target found at {}", base)
}

async fn fetch_targets(http: &reqwest::Client, list_url: reqwest::Url) -> Option<Vec<Value>> {
    let resp = http.get(list_url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    Some(resp.json().await.unwrap_or_default())
}

pub(super) fn retry_attempts() -> std::ops::Range<usize> {
    0..10
}

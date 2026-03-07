use std::time::Duration;

use anyhow::Context;
use serde_json::Value;

use super::super::CdpConnection;

pub(crate) async fn wait_for_load(cdp: &CdpConnection) {
    for _ in 0..super::super::super::PAGE_LOAD_MAX_POLLS {
        if let Ok(val) = cdp.run_js("document.readyState").await {
            let state = val.as_str().unwrap_or("");
            if state == "complete" || state == "interactive" {
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(
            super::super::super::PAGE_LOAD_POLL_INTERVAL_MS,
        ))
        .await;
    }
}

pub(crate) async fn page_info(cdp: &CdpConnection) -> anyhow::Result<Value> {
    let info = cdp
        .run_js("JSON.stringify({title: document.title, url: location.href})")
        .await
        .context("page info js")?;
    let parsed: Value = info
        .as_str()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(info);

    Ok(serde_json::json!({
        "title": parsed.get("title").cloned().unwrap_or(Value::Null),
        "url": parsed.get("url").cloned().unwrap_or(Value::Null),
    }))
}

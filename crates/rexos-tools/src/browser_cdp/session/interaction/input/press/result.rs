use serde_json::Value;

use crate::browser_cdp::session::helpers::page_info;
use crate::browser_cdp::session::CdpBrowserSession;

pub(super) async fn key_result(
    session: &CdpBrowserSession,
    selector: Option<&str>,
    key: &str,
) -> anyhow::Result<Value> {
    let info = page_info(&session.cdp)
        .await
        .unwrap_or_else(|_| serde_json::json!({}));
    let mut obj = info.as_object().cloned().unwrap_or_default();
    obj.insert("key".to_string(), Value::String(key.to_string()));
    obj.insert(
        "selector".to_string(),
        selector
            .map(|value| Value::String(value.to_string()))
            .unwrap_or(Value::Null),
    );
    Ok(Value::Object(obj))
}

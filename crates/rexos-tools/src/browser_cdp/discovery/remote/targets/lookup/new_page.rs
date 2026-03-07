use anyhow::Context;
use serde_json::Value;

pub(super) async fn try_new_page_ws(
    http: &reqwest::Client,
    base: &reqwest::Url,
) -> anyhow::Result<Option<String>> {
    let new_url = base.join("/json/new").context("join /json/new")?;
    if let Ok(resp) = http.get(new_url).send().await {
        if resp.status().is_success() {
            let value: Value = resp.json().await.unwrap_or(Value::Null);
            if let Some(ws) = value
                .get("webSocketDebuggerUrl")
                .and_then(|value| value.as_str())
            {
                return Ok(Some(ws.to_string()));
            }
        }
    }
    Ok(None)
}

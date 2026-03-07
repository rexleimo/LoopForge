use anyhow::bail;
use serde_json::Value;

use super::super::CdpConnection;

impl CdpConnection {
    pub(crate) async fn run_js(&self, expression: &str) -> anyhow::Result<Value> {
        let result = self
            .send(
                "Runtime.evaluate",
                serde_json::json!({
                    "expression": expression,
                    "returnByValue": true,
                    "awaitPromise": true,
                }),
            )
            .await?;

        extract_js_value(&result)
    }
}

fn extract_js_value(result: &Value) -> anyhow::Result<Value> {
    if let Some(description) = result
        .get("exceptionDetails")
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
    {
        bail!("JS error: {description}");
    }

    Ok(result
        .get("result")
        .and_then(|value| value.get("value"))
        .cloned()
        .unwrap_or(Value::Null))
}

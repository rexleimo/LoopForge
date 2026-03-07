use serde_json::Value;

pub(super) fn response_error(json: &Value) -> Option<anyhow::Error> {
    json.get("error").map(|error| {
        let msg = error
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("CDP error")
            .to_string();
        anyhow::anyhow!(msg)
    })
}

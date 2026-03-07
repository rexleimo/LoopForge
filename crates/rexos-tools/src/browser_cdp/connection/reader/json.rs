pub(super) fn response_id(json: &serde_json::Value) -> Option<u64> {
    json.get("id").and_then(|value| value.as_u64())
}

pub(super) fn response_result(json: &serde_json::Value) -> serde_json::Value {
    json.get("result")
        .cloned()
        .unwrap_or(serde_json::Value::Null)
}

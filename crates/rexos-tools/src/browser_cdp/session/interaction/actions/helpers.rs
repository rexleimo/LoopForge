use anyhow::bail;
use serde_json::Value;

pub(super) fn parse_action_result(value: Value) -> Value {
    value
        .as_str()
        .and_then(|text| serde_json::from_str(text).ok())
        .unwrap_or(value)
}

pub(super) fn ensure_action_success(parsed: &Value, fallback_message: &str) -> anyhow::Result<()> {
    if parsed["success"].as_bool() == Some(false) {
        let message = parsed["error"]
            .as_str()
            .unwrap_or(fallback_message)
            .to_string();
        bail!(message);
    }
    Ok(())
}

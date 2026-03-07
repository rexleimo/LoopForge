use serde_json::Value;

use super::super::super::helpers::{insert_optional_string, insert_optional_u64};

pub(super) fn press_key_request(selector: Option<&str>, key: &str) -> Value {
    let mut cmd = serde_json::json!({
        "action": "PressKey",
        "key": key,
    });
    insert_optional_string(&mut cmd, "selector", selector);
    cmd
}

pub(super) fn wait_for_request(
    selector: Option<&str>,
    text: Option<&str>,
    timeout_ms: Option<u64>,
) -> Value {
    let mut cmd = serde_json::json!({ "action": "WaitFor" });
    insert_optional_string(&mut cmd, "selector", selector);
    insert_optional_string(&mut cmd, "text", text);
    insert_optional_u64(&mut cmd, "timeout_ms", timeout_ms);
    cmd
}

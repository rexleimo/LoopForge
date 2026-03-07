use serde_json::json;

use super::payload::{press_key_request, wait_for_request};

#[test]
fn press_key_request_keeps_selector_when_present() {
    assert_eq!(
        press_key_request(Some("#name"), "Enter"),
        json!({
            "action": "PressKey",
            "key": "Enter",
            "selector": "#name",
        })
    );
}

#[test]
fn wait_for_request_keeps_optional_text_and_timeout() {
    assert_eq!(
        wait_for_request(Some(".item"), Some("ready"), Some(2500)),
        json!({
            "action": "WaitFor",
            "selector": ".item",
            "text": "ready",
            "timeout_ms": 2500,
        })
    );
}

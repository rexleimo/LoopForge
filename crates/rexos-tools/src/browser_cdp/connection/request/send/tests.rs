use serde_json::json;

use super::message::request_message_text;
use super::response::{response_channel_closed_error, response_timeout_error};

#[test]
fn request_message_text_serializes_id_method_and_params() {
    let text = request_message_text(7, "Runtime.evaluate", json!({"expression": "1+1"}));
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        value,
        json!({
            "id": 7,
            "method": "Runtime.evaluate",
            "params": {"expression": "1+1"}
        })
    );
}

#[test]
fn response_error_helpers_keep_stable_messages() {
    assert_eq!(
        response_channel_closed_error().to_string(),
        "CDP response channel closed"
    );
    assert_eq!(
        response_timeout_error().to_string(),
        "CDP command timed out"
    );
}

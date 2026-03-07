use serde_json::json;

use super::payload::{back_request, navigate_request, scroll_request};

#[test]
fn navigate_request_keeps_action_and_url() {
    assert_eq!(
        navigate_request("https://example.com"),
        json!({
            "action": "Navigate",
            "url": "https://example.com",
        })
    );
}

#[test]
fn back_request_keeps_action_only() {
    assert_eq!(back_request(), json!({ "action": "Back" }));
}

#[test]
fn scroll_request_keeps_direction_and_amount() {
    assert_eq!(
        scroll_request("down", 320),
        json!({
            "action": "Scroll",
            "direction": "down",
            "amount": 320,
        })
    );
}

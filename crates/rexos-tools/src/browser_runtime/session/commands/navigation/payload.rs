use serde_json::Value;

pub(super) fn navigate_request(url: &str) -> Value {
    serde_json::json!({
        "action": "Navigate",
        "url": url,
    })
}

pub(super) fn back_request() -> Value {
    serde_json::json!({ "action": "Back" })
}

pub(super) fn scroll_request(direction: &str, amount: i64) -> Value {
    serde_json::json!({
        "action": "Scroll",
        "direction": direction,
        "amount": amount,
    })
}

pub(super) fn key_event_payload(
    event_type: &str,
    key: &str,
    code: &str,
    vkey: i32,
) -> serde_json::Value {
    serde_json::json!({
        "type": event_type,
        "key": key,
        "code": code,
        "windowsVirtualKeyCode": vkey,
        "nativeVirtualKeyCode": vkey,
    })
}

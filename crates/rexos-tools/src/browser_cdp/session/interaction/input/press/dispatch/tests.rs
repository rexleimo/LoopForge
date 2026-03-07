use serde_json::json;

use super::{fallback_key_script, key_event_payload};

#[test]
fn key_event_payload_includes_native_and_windows_key_codes() {
    assert_eq!(
        key_event_payload("keyDown", "Enter", "Enter", 13),
        json!({
            "type": "keyDown",
            "key": "Enter",
            "code": "Enter",
            "windowsVirtualKeyCode": 13,
            "nativeVirtualKeyCode": 13,
        })
    );
}

#[test]
fn fallback_key_script_keeps_submit_behavior_for_enter() {
    let js = fallback_key_script("Enter");
    assert!(js.contains("requestSubmit"));
    assert!(js.contains("KeyboardEvent('keydown'"));
    assert!(js.contains("\"Enter\""));
}

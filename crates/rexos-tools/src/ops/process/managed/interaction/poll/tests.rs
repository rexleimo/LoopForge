use serde_json::json;

use super::response::process_poll_payload;
use super::status::alive_from_exit_code;

#[test]
fn alive_from_exit_code_reflects_running_state() {
    assert!(alive_from_exit_code(None));
    assert!(!alive_from_exit_code(Some(0)));
}

#[test]
fn process_poll_payload_keeps_stdio_and_status_fields() {
    assert_eq!(
        process_poll_payload("out", "err", true, false, Some(3), true),
        json!({
            "stdout": "out",
            "stderr": "err",
            "stdout_truncated": true,
            "stderr_truncated": false,
            "exit_code": 3,
            "alive": true,
        })
    );
}

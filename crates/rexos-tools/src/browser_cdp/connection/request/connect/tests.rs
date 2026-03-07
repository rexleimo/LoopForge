use super::error::{connect_error_message, timeout_error_message};
use super::stream::pending_map;

#[test]
fn cdp_connect_timeout_error_message_keeps_url() {
    let url = "ws://127.0.0.1:9222/devtools/browser/demo";
    assert_eq!(
        timeout_error_message(url),
        format!("CDP WebSocket connect timed out: {url}")
    );
}

#[test]
fn cdp_connect_error_message_keeps_inner_error_text() {
    let msg = connect_error_message("boom");
    assert!(msg.contains("CDP WebSocket connect failed: boom"));
    assert!(pending_map().is_empty());
}

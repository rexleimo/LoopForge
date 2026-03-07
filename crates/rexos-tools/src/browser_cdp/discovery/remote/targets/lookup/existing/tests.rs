use serde_json::json;

use super::page::page_target_ws;
use super::poll::retry_attempts;

#[test]
fn page_target_ws_ignores_non_page_targets_and_returns_page_socket() {
    let targets = vec![
        json!({ "type": "service_worker", "webSocketDebuggerUrl": "ws://ignored" }),
        json!({ "type": "page", "webSocketDebuggerUrl": "ws://page-1" }),
    ];

    assert_eq!(page_target_ws(&targets), Some("ws://page-1".to_string()));
}

#[test]
fn retry_attempts_count_stays_constant() {
    assert_eq!(retry_attempts().count(), 10);
}

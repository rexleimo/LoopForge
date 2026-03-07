use serde_json::json;

use super::response::{process_start_payload, process_start_status};

#[test]
fn process_start_payload_reports_started_status() {
    assert_eq!(
        process_start_payload("abc"),
        json!({
            "process_id": "abc",
            "status": "started"
        })
    );
}

#[test]
fn process_start_status_stays_constant() {
    assert_eq!(process_start_status(), "started");
}

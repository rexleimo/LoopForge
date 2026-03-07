pub(super) fn process_start_status() -> &'static str {
    "started"
}

pub(super) fn process_start_payload(process_id: &str) -> serde_json::Value {
    serde_json::json!({
        "process_id": process_id,
        "status": process_start_status(),
    })
}

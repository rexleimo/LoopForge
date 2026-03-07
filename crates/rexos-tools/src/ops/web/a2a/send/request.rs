pub(super) fn send_request(message: &str, session_id: Option<&str>) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tasks/send",
        "params": {
            "message": {
                "role": "user",
                "parts": [{ "type": "text", "text": message }]
            },
            "sessionId": session_id,
        }
    })
}

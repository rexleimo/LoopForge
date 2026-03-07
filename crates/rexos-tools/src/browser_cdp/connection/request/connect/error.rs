pub(super) fn timeout_error_message(ws_url: &str) -> String {
    format!("CDP WebSocket connect timed out: {ws_url}")
}

pub(super) fn connect_error_message(error: impl std::fmt::Display) -> String {
    format!("CDP WebSocket connect failed: {error}")
}

use super::{encoded_bridge_command, trimmed_bridge_response_line};

#[test]
fn encoded_bridge_command_appends_newline() {
    let line = encoded_bridge_command(&serde_json::json!({"action": "Ping"})).unwrap();
    assert!(line.ends_with('\n'));
    assert!(line.contains("\"Ping\""));
}

#[test]
fn trimmed_bridge_response_line_removes_outer_newlines() {
    assert_eq!(
        trimmed_bridge_response_line(" {\"ok\":true}\n"),
        "{\"ok\":true}"
    );
}

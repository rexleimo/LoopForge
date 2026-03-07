use super::parse::trimmed_bridge_response_line;

#[test]
fn trimmed_bridge_response_line_trims_outer_whitespace() {
    assert_eq!(
        trimmed_bridge_response_line(" {\"ok\":true}\n"),
        "{\"ok\":true}"
    );
}

use super::{is_field_input_tool, is_script_input_tool, is_scroll_input_tool, is_wait_input_tool};

#[test]
fn grouped_input_tool_classifiers_cover_expected_names() {
    assert!(is_field_input_tool("browser_click"));
    assert!(is_field_input_tool("browser_press_key"));
    assert!(is_wait_input_tool("browser_wait_for"));
    assert!(is_scroll_input_tool("browser_scroll"));
    assert!(is_script_input_tool("browser_run_js"));
    assert!(!is_wait_input_tool("browser_scroll"));
}

pub(super) fn is_field_input_tool(name: &str) -> bool {
    matches!(name, "browser_click" | "browser_type" | "browser_press_key")
}

pub(super) fn is_wait_input_tool(name: &str) -> bool {
    matches!(name, "browser_wait" | "browser_wait_for")
}

pub(super) fn is_scroll_input_tool(name: &str) -> bool {
    matches!(name, "browser_scroll")
}

pub(super) fn is_script_input_tool(name: &str) -> bool {
    matches!(name, "browser_run_js")
}

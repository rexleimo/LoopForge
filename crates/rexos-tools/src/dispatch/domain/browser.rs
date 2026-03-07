pub(super) fn is_browser_tool(name: &str) -> bool {
    matches!(
        name,
        "browser_navigate"
            | "browser_back"
            | "browser_close"
            | "browser_click"
            | "browser_type"
            | "browser_press_key"
            | "browser_scroll"
            | "browser_wait"
            | "browser_wait_for"
            | "browser_read_page"
            | "browser_run_js"
            | "browser_screenshot"
    )
}

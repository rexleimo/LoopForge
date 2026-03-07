use super::*;

#[test]
fn tool_definitions_include_browser_tools() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let defs = tools
        .definitions()
        .into_iter()
        .map(|d| d.function.name)
        .collect::<std::collections::BTreeSet<_>>();

    for name in [
        "browser_navigate",
        "browser_back",
        "browser_scroll",
        "browser_click",
        "browser_type",
        "browser_press_key",
        "browser_wait",
        "browser_wait_for",
        "browser_read_page",
        "browser_run_js",
        "browser_screenshot",
        "browser_close",
    ] {
        assert!(defs.contains(name), "missing tool definition: {name}");
    }
}

#[test]
fn browser_backend_default_honors_aliases() {
    let _guard = ENV_LOCK.lock().unwrap();
    let _unset = std::env::var_os("LOOPFORGE_BROWSER_BACKEND");

    std::env::set_var("LOOPFORGE_BROWSER_BACKEND", "bridge");
    assert_eq!(
        crate::browser_runtime::browser_backend_default(),
        crate::browser_runtime::BrowserBackend::Playwright
    );

    std::env::set_var("LOOPFORGE_BROWSER_BACKEND", "chromium");
    assert_eq!(
        crate::browser_runtime::browser_backend_default(),
        crate::browser_runtime::BrowserBackend::Cdp
    );

    std::env::remove_var("LOOPFORGE_BROWSER_BACKEND");
}

#[test]
fn browser_bridge_script_includes_back_scroll_and_run_js_actions() {
    for needle in ["\"Back\"", "\"Scroll\"", "\"RunJs\""] {
        assert!(
            crate::BROWSER_BRIDGE_SCRIPT.contains(needle),
            "bridge script missing action handler: {needle}"
        );
    }
}

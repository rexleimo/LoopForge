pub(super) fn reuse_existing_tab() -> bool {
    let tab_mode = std::env::var("LOOPFORGE_BROWSER_CDP_TAB_MODE")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .unwrap_or_else(|| "new".to_string());
    matches!(tab_mode.as_str(), "reuse" | "existing" | "list")
}

pub(super) fn base_chromium_args(port: u16, user_data_dir: &std::path::Path) -> Vec<String> {
    let user_data_dir_arg = user_data_dir.to_string_lossy().to_string();
    vec![
        format!("--remote-debugging-port={port}"),
        "--remote-debugging-address=127.0.0.1".to_string(),
        "--no-first-run".to_string(),
        "--no-default-browser-check".to_string(),
        "--disable-extensions".to_string(),
        "--disable-background-networking".to_string(),
        "--disable-sync".to_string(),
        "--disable-translate".to_string(),
        "--disable-features=TranslateUI".to_string(),
        "--metrics-recording-only".to_string(),
        "--disable-popup-blocking".to_string(),
        "--window-size=1280,720".to_string(),
        format!("--user-data-dir={user_data_dir_arg}"),
        "about:blank".to_string(),
    ]
}

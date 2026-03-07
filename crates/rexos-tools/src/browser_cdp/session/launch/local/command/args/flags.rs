pub(super) fn with_headless_args(mut args: Vec<String>) -> Vec<String> {
    args.insert(0, "--headless=new".to_string());
    args.push("--disable-gpu".to_string());
    args
}

pub(super) fn append_no_sandbox_args(args: &mut Vec<String>) {
    args.push("--no-sandbox".to_string());
    args.push("--disable-setuid-sandbox".to_string());
}

pub(super) fn no_sandbox_enabled() -> bool {
    std::env::var("LOOPFORGE_BROWSER_NO_SANDBOX")
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

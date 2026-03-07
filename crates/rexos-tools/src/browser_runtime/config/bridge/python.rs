pub(super) fn browser_python_exe() -> String {
    if let Ok(value) = std::env::var("LOOPFORGE_BROWSER_PYTHON") {
        if !value.trim().is_empty() {
            return value;
        }
    }
    if cfg!(windows) {
        "python".to_string()
    } else {
        "python3".to_string()
    }
}

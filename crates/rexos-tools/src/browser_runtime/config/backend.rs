#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BrowserBackend {
    Cdp,
    Playwright,
}

pub(crate) fn browser_backend_default() -> BrowserBackend {
    if let Ok(v) = std::env::var("LOOPFORGE_BROWSER_BACKEND") {
        match v.trim().to_ascii_lowercase().as_str() {
            "cdp" | "native" | "chromium" => return BrowserBackend::Cdp,
            "playwright" | "bridge" | "python" => return BrowserBackend::Playwright,
            _ => {}
        }
    }
    BrowserBackend::Cdp
}

pub(crate) fn browser_headless_default() -> bool {
    if let Ok(v) = std::env::var("LOOPFORGE_BROWSER_HEADLESS") {
        match v.trim().to_ascii_lowercase().as_str() {
            "0" | "false" | "no" | "off" => return false,
            "1" | "true" | "yes" | "on" => return true,
            _ => {}
        }
    }
    true
}

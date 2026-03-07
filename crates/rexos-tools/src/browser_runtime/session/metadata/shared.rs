use crate::browser_runtime::BrowserBackend;

pub(super) fn backend_label(backend: BrowserBackend) -> &'static str {
    match backend {
        BrowserBackend::Cdp => "cdp",
        BrowserBackend::Playwright => "playwright",
    }
}

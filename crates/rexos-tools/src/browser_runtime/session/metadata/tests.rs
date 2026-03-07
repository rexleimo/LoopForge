use crate::browser_runtime::BrowserBackend;

use super::backend_label;

#[test]
fn backend_label_matches_runtime_backend_names() {
    assert_eq!(backend_label(BrowserBackend::Cdp), "cdp");
    assert_eq!(backend_label(BrowserBackend::Playwright), "playwright");
}

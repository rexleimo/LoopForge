use reqwest::Url;

use super::{allowed_special_browser_url, browser_url_host_port};

#[test]
fn allowed_special_browser_url_recognizes_internal_pages() {
    assert!(allowed_special_browser_url(
        &Url::parse("about:blank").unwrap()
    ));
    assert!(allowed_special_browser_url(
        &Url::parse("chrome-error://chromewebdata/").unwrap()
    ));
    assert!(!allowed_special_browser_url(
        &Url::parse("https://example.com").unwrap()
    ));
}

#[test]
fn browser_url_host_port_extracts_network_target() {
    assert_eq!(
        browser_url_host_port(&Url::parse("https://example.com/path").unwrap()).unwrap(),
        ("example.com".to_string(), 443)
    );
}

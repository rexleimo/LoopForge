use super::super::discovery::validate_remote_cdp_base_url;
use super::shared::ENV_LOCK;

#[test]
fn validate_remote_cdp_base_url_rejects_non_loopback_by_default() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    std::env::remove_var("LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE");
    let url = reqwest::Url::parse("http://example.com:9222").unwrap();
    let err = validate_remote_cdp_base_url(&url).unwrap_err();
    assert!(err.to_string().contains("ALLOW_REMOTE"), "{err}");
}

#[test]
fn validate_remote_cdp_base_url_allows_loopback() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    std::env::remove_var("LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE");
    let url = reqwest::Url::parse("http://127.0.0.1:9222").unwrap();
    validate_remote_cdp_base_url(&url).unwrap();
}

#[test]
fn validate_remote_cdp_base_url_allows_non_loopback_with_opt_in() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    std::env::set_var("LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE", "1");
    let url = reqwest::Url::parse("http://example.com:9222").unwrap();
    validate_remote_cdp_base_url(&url).unwrap();
    std::env::remove_var("LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE");
}

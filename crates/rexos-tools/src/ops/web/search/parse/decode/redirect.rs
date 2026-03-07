pub(super) fn duckduckgo_redirect_target(url: &str) -> Option<&str> {
    url.split("uddg=")
        .nth(1)
        .and_then(|value| value.split('&').next())
}

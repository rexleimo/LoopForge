use super::super::*;

#[tokio::test]
async fn ensure_browser_url_allowed_rejects_file_scheme_even_when_allow_private_true() {
    let err = ensure_browser_url_allowed("file:///etc/passwd", true)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("http/https"), "{err}");
}

#[tokio::test]
async fn ensure_browser_url_allowed_allows_about_blank() {
    ensure_browser_url_allowed("about:blank", false)
        .await
        .unwrap();
}

#[tokio::test]
async fn ensure_browser_url_allowed_allows_chrome_error_page() {
    ensure_browser_url_allowed("chrome-error://chromewebdata/", false)
        .await
        .unwrap();
}

#[tokio::test]
async fn ensure_browser_url_allowed_allows_public_ip_http() {
    ensure_browser_url_allowed("http://1.1.1.1", false)
        .await
        .unwrap();
}

#[tokio::test]
async fn browser_navigate_denies_loopback_by_default() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let err = tools
        .call(
            "browser_navigate",
            r#"{ "url": "http://127.0.0.1:1/", "allow_private": false }"#,
        )
        .await
        .unwrap_err();

    let message = err.to_string();
    assert!(
        message.contains("loopback") || message.contains("private") || message.contains("denied"),
        "{message}"
    );
}

#[tokio::test]
async fn browser_navigate_allows_loopback_when_allow_private_true() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let result = tools
        .call(
            "browser_navigate",
            r#"{ "url": "http://127.0.0.1:1/", "allow_private": true }"#,
        )
        .await;

    match result {
        Ok(output) => assert!(!output.trim().is_empty()),
        Err(err) => {
            let message = err.to_string();
            assert!(
                !message.contains("loopback/private address"),
                "unexpected SSRF-style error: {message}"
            );
        }
    }
}

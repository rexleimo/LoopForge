use super::super::*;

#[tokio::test]
async fn browser_click_requires_session() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let err = tools
        .call("browser_click", r#"{ "selector": "a" }"#)
        .await
        .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("browser_navigate") || message.contains("session"),
        "{message}"
    );
}

#[tokio::test]
async fn browser_press_key_requires_session() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let err = tools
        .call("browser_press_key", r#"{ "key": "Enter" }"#)
        .await
        .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("browser_navigate") || message.contains("session"),
        "{message}"
    );
}

#[tokio::test]
async fn browser_wait_for_requires_session() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let err = tools
        .call("browser_wait_for", r#"{ "text": "hello" }"#)
        .await
        .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("browser_navigate") || message.contains("session"),
        "{message}"
    );
}

#[tokio::test]
async fn browser_read_page_requires_session() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let err = tools.call("browser_read_page", r#"{}"#).await.unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("browser_navigate") || message.contains("session"),
        "{message}"
    );
}

#[tokio::test]
async fn browser_screenshot_requires_session() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let err = tools
        .call("browser_screenshot", r#"{ "path": "shot.png" }"#)
        .await
        .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("browser_navigate") || message.contains("session"),
        "{message}"
    );
}

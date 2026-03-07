use super::super::*;

#[tokio::test]
async fn browser_close_is_idempotent() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let out = tools.call("browser_close", r#"{}"#).await.unwrap();
    assert_eq!(out.trim(), "ok");
}

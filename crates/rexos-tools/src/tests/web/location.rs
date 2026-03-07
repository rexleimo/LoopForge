use super::*;

#[tokio::test]
async fn location_get_returns_environment_metadata() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let out = tools.call("location_get", r#"{}"#).await.unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("location_get is json");
    assert_eq!(
        v.get("os").and_then(|v| v.as_str()),
        Some(std::env::consts::OS),
        "{v}"
    );
    assert_eq!(
        v.get("arch").and_then(|v| v.as_str()),
        Some(std::env::consts::ARCH),
        "{v}"
    );
}

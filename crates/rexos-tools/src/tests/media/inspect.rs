use super::*;

#[tokio::test]
async fn media_describe_returns_basic_metadata() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    std::fs::write(workspace.join("audio.wav"), b"RIFF....WAVEfmt ").unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("media_describe", r#"{ "path": "audio.wav" }"#)
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("media_describe is json");
    assert_eq!(v.get("path").and_then(|v| v.as_str()), Some("audio.wav"));
    assert_eq!(v.get("bytes").and_then(|v| v.as_u64()), Some(16));
    assert_eq!(v.get("kind").and_then(|v| v.as_str()), Some("audio"));
}

#[tokio::test]
async fn image_analyze_returns_dimensions_for_png() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let png_1x1 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+X2OQAAAAASUVORK5CYII=";
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(png_1x1)
        .expect("decode png base64");
    std::fs::write(workspace.join("img.png"), bytes).unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("image_analyze", r#"{ "path": "img.png" }"#)
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("image_analyze is json");
    assert_eq!(v.get("width").and_then(|v| v.as_u64()), Some(1), "{v}");
    assert_eq!(v.get("height").and_then(|v| v.as_u64()), Some(1), "{v}");
}

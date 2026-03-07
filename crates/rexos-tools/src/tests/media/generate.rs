use super::*;

#[tokio::test]
async fn text_to_speech_writes_wav_file() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace.clone()).unwrap();
    let out = tools
        .call(
            "text_to_speech",
            r#"{ "text": "hello", "path": "out.wav" }"#,
        )
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("text_to_speech output is json");
    assert_eq!(v.get("path").and_then(|v| v.as_str()), Some("out.wav"));
    assert_eq!(v.get("format").and_then(|v| v.as_str()), Some("wav"));

    let bytes = std::fs::read(workspace.join("out.wav")).unwrap();
    assert!(bytes.starts_with(b"RIFF"), "missing RIFF header");
    assert!(
        bytes.windows(4).any(|w| w == b"WAVE"),
        "missing WAVE header"
    );
}

#[tokio::test]
async fn image_generate_writes_svg_file() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace.clone()).unwrap();
    let out = tools
        .call(
            "image_generate",
            r#"{ "prompt": "hello", "path": "out.svg" }"#,
        )
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("image_generate is json");
    assert_eq!(v.get("path").and_then(|v| v.as_str()), Some("out.svg"));
    assert_eq!(v.get("format").and_then(|v| v.as_str()), Some("svg"));

    let svg = std::fs::read_to_string(workspace.join("out.svg")).unwrap();
    assert!(svg.starts_with("<svg"), "{svg}");
    assert!(svg.contains("hello"), "{svg}");
}

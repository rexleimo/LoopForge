use super::*;

#[tokio::test]
async fn media_transcribe_reads_text_transcripts() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    std::fs::write(workspace.join("transcript.txt"), "hello world").unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("media_transcribe", r#"{ "path": "transcript.txt" }"#)
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("media_transcribe output is json");
    assert_eq!(v.get("text").and_then(|v| v.as_str()), Some("hello world"));
}

#[tokio::test]
async fn speech_to_text_reads_text_transcripts() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    std::fs::write(workspace.join("transcript.txt"), "hello world").unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("speech_to_text", r#"{ "path": "transcript.txt" }"#)
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("speech_to_text output is json");
    assert_eq!(
        v.get("transcript").and_then(|v| v.as_str()),
        Some("hello world")
    );
    assert_eq!(v.get("text").and_then(|v| v.as_str()), Some("hello world"));
}

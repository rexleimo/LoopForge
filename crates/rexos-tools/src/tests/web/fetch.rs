use super::*;

#[tokio::test]
async fn web_fetch_truncation_preserves_head_and_tail() {
    async fn handler() -> String {
        let head = "HEAD_MARKER";
        let tail = "TAIL_MARKER";
        let filler = "A".repeat(5000);
        format!("{head}\n{filler}\n{tail}\n")
    }

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    let out = tools
        .call(
            "web_fetch",
            &serde_json::json!({
                "url": format!("http://{addr}/"),
                "allow_private": true,
                "max_bytes": 200,
            })
            .to_string(),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let body = v.get("body").and_then(|v| v.as_str()).unwrap_or("");

    assert!(body.contains("HEAD_MARKER"), "{body}");
    assert!(body.contains("TAIL_MARKER"), "{body}");
    assert_eq!(
        v.get("truncated").and_then(|v| v.as_bool()),
        Some(true),
        "{v}"
    );

    server.abort();
}

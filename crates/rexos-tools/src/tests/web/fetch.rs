use super::*;
use rexos_kernel::security::{EgressConfig, EgressRule, SecurityConfig};

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

#[tokio::test]
async fn web_fetch_respects_egress_policy_rules() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, Router::new().route("/", get(|| async { "ok" })))
            .await
            .unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new_with_security_config(
        tmp.path().to_path_buf(),
        SecurityConfig {
            egress: EgressConfig {
                rules: vec![EgressRule {
                    tool: "web_fetch".to_string(),
                    host: "example.com".to_string(),
                    path_prefix: "/".to_string(),
                    methods: vec!["GET".to_string()],
                }],
            },
            ..Default::default()
        },
    )
    .unwrap();

    let err = tools
        .call(
            "web_fetch",
            &serde_json::json!({
                "url": format!("http://{addr}/"),
                "allow_private": true,
                "max_bytes": 100,
            })
            .to_string(),
        )
        .await
        .unwrap_err();
    assert!(err.to_string().contains("host"), "{err}");

    server.abort();
}

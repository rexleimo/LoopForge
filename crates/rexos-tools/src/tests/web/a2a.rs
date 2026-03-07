use super::*;

#[tokio::test]
async fn a2a_discover_denies_loopback_by_default() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let err = tools
        .call("a2a_discover", r#"{ "url": "http://127.0.0.1:1/" }"#)
        .await
        .unwrap_err();

    let msg = err.to_string();
    assert!(
        msg.contains("loopback") || msg.contains("private") || msg.contains("denied"),
        "{msg}"
    );
}

#[tokio::test]
async fn a2a_discover_fetches_agent_card_when_allow_private_true() {
    async fn handler() -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "name": "demo-agent",
            "description": "demo",
            "url": "http://example.invalid/a2a",
            "version": "1.0",
            "capabilities": { "streaming": false, "pushNotifications": false, "stateTransitionHistory": false },
            "skills": [],
            "defaultInputModes": ["text"],
            "defaultOutputModes": ["text"]
        }))
    }

    let app = Router::new().route("/.well-known/agent.json", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let out = tools
        .call(
            "a2a_discover",
            &format!(
                r#"{{ "url": "http://127.0.0.1:{}/", "allow_private": true }}"#,
                addr.port()
            ),
        )
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("a2a_discover output is json");
    assert_eq!(v.get("name").and_then(|v| v.as_str()), Some("demo-agent"));

    server.abort();
}

#[derive(Clone, Default)]
struct A2aSendState {
    last_method: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[tokio::test]
async fn a2a_send_posts_jsonrpc_and_returns_result() {
    async fn handler(
        State(state): State<A2aSendState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let method = payload
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        *state.last_method.lock().unwrap() = Some(method.clone());

        if method != "tasks/send" {
            return Json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": { "message": "unexpected method" }
            }));
        }

        Json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "id": "task-1",
                "status": "Completed",
                "messages": [{"role":"agent","parts":[{"type":"text","text":"ok"}]}],
                "artifacts": []
            }
        }))
    }

    let state = A2aSendState::default();
    let app = Router::new()
        .route("/a2a", post(handler))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let out = tools
        .call(
            "a2a_send",
            &format!(
                r#"{{ "agent_url": "http://127.0.0.1:{}/a2a", "message": "hello", "allow_private": true }}"#,
                addr.port()
            ),
        )
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("a2a_send output is json");
    assert_eq!(v.get("id").and_then(|v| v.as_str()), Some("task-1"), "{v}");
    assert_eq!(
        state.last_method.lock().unwrap().as_deref(),
        Some("tasks/send")
    );

    server.abort();
}

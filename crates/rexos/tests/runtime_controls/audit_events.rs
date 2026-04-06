use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use super::common::{test_agent, TestState};

#[tokio::test]
async fn tool_audit_marks_truncated_for_large_output() {
    async fn handler(
        State(state): State<TestState>,
        Json(_payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let mut calls = state.calls.lock().unwrap();
        *calls += 1;

        if *calls == 1 {
            return Json(json!({
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": null,
                        "tool_calls": [{
                            "id": "call_1",
                            "type": "function",
                            "function": {
                                "name": "fs_read",
                                "arguments": "{\"path\":\"large.txt\"}"
                            }
                        }]
                    },
                    "finish_reason": "tool_calls"
                }]
            }));
        }

        Json(json!({
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "done" },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(
        workspace.join("large.txt"),
        format!("HEAD:{}:TAIL", "x".repeat(25_000)),
    )
    .unwrap();

    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = test_agent(format!("http://{addr}/v1"), memory);

    let out = agent
        .run_session(
            workspace,
            "s-truncate-audit",
            None,
            "read large file",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let raw = memory2
        .kv_get("rexos.audit.tool_calls")
        .unwrap()
        .unwrap_or_default();
    let events: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = events.as_array().unwrap();
    let event = arr
        .iter()
        .rev()
        .find(|v| v["session_id"] == "s-truncate-audit")
        .expect("expected audit event");
    assert_eq!(event["tool_name"], "fs_read");
    assert_eq!(event["success"], true);
    assert_eq!(event["truncated"], true);
    assert!(event["duration_ms"].as_u64().is_some());

    server.abort();
}

#[tokio::test]
async fn acp_events_capture_session_and_tool_lifecycle() {
    async fn handler(
        State(state): State<TestState>,
        Json(_payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let mut calls = state.calls.lock().unwrap();
        *calls += 1;

        if *calls == 1 {
            return Json(json!({
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": null,
                        "tool_calls": [{
                            "id": "call_1",
                            "type": "function",
                            "function": {
                                "name": "fs_read",
                                "arguments": "{\"path\":\"hello.txt\"}"
                            }
                        }]
                    },
                    "finish_reason": "tool_calls"
                }]
            }));
        }

        Json(json!({
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "done" },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(workspace.join("hello.txt"), "hello").unwrap();

    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = test_agent(format!("http://{addr}/v1"), memory);

    let out = agent
        .run_session(
            workspace,
            "s-acp-events",
            None,
            "read file",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let raw = memory2
        .kv_get("rexos.acp.events")
        .unwrap()
        .unwrap_or_default();
    let events: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = events.as_array().unwrap();
    assert!(
        arr.iter()
            .any(|v| v["session_id"] == "s-acp-events" && v["event_type"] == "session.started"),
        "missing session.started: {events}"
    );
    assert!(
        arr.iter()
            .any(|v| v["session_id"] == "s-acp-events" && v["event_type"] == "tool.succeeded"),
        "missing tool.succeeded: {events}"
    );
    assert!(
        arr.iter()
            .any(|v| v["session_id"] == "s-acp-events" && v["event_type"] == "session.completed"),
        "missing session.completed: {events}"
    );

    server.abort();
}

#[tokio::test]
async fn delivery_checkpoint_is_written_after_dispatch() {
    async fn handler(
        State(state): State<TestState>,
        Json(_payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let mut calls = state.calls.lock().unwrap();
        *calls += 1;

        if *calls == 1 {
            return Json(json!({
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": null,
                        "tool_calls": [{
                            "id": "call_1",
                            "type": "function",
                            "function": {
                                "name": "channel_send",
                                "arguments": "{\"channel\":\"console\",\"recipient\":\"user1\",\"message\":\"hi\"}"
                            }
                        }]
                    },
                    "finish_reason": "tool_calls"
                }]
            }));
        }

        Json(json!({
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "done" },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = test_agent(format!("http://{addr}/v1"), memory);

    let out = agent
        .run_session(
            workspace,
            "s-checkpoint",
            None,
            "send",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let dispatcher = rexos::agent::OutboxDispatcher::new(
        rexos::memory::MemoryStore::open_or_create(&paths).unwrap(),
    )
    .unwrap();
    let summary = dispatcher.drain_once(10).await.unwrap();
    assert_eq!(summary.sent, 1);

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let raw = memory2
        .kv_get("rexos.acp.checkpoints.s-checkpoint")
        .unwrap()
        .unwrap_or_default();
    let checkpoints: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = checkpoints
        .as_array()
        .expect("acp checkpoints should be an array");
    assert!(
        arr.iter().any(|v| v["channel"] == "console"),
        "missing console checkpoint: {checkpoints}"
    );

    server.abort();
}

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

#[derive(Clone, Default)]
struct TestState {
    calls: Arc<Mutex<u32>>,
}

fn test_agent(
    base_url: String,
    memory: rexos::memory::MemoryStore,
) -> rexos::agent::AgentRuntime {
    let mut providers = BTreeMap::new();
    providers.insert(
        "ollama".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url,
            api_key_env: "".to_string(),
            default_model: "x".to_string(),
        },
    );

    let cfg = rexos::config::RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers,
        router: rexos::config::RouterConfig::default(),
    };
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg).unwrap();
    let router = rexos::router::ModelRouter::new(rexos::config::RouterConfig {
        planning: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
        coding: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
        summary: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
    });
    rexos::agent::AgentRuntime::new(memory, llms, router)
}

#[tokio::test]
async fn session_tool_whitelist_blocks_tool_and_audits_failure() {
    async fn handler(
        State(state): State<TestState>,
        Json(_payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let mut calls = state.calls.lock().unwrap();
        *calls += 1;

        Json(json!({
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "fs_write",
                            "arguments": "{\"path\":\"x.txt\",\"content\":\"blocked\"}"
                        }
                    }]
                },
                "finish_reason": "tool_calls"
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
        base_dir: tmp.path().join(".rexos"),
    };
    paths.ensure_dirs().unwrap();
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();

    let agent = test_agent(format!("http://{addr}/v1"), memory);
    agent
        .set_session_allowed_tools("s-whitelist", vec!["fs_read".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace.clone(),
            "s-whitelist",
            None,
            "try write",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("tool not allowed"),
        "expected tool deny error, got: {err_text}"
    );

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let raw = memory2
        .kv_get("rexos.audit.tool_calls")
        .unwrap()
        .unwrap_or_default();
    let events: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = events.as_array().unwrap();
    let last = arr.last().unwrap();
    assert_eq!(last["session_id"], "s-whitelist");
    assert_eq!(last["tool_name"], "fs_write");
    assert_eq!(last["success"], false);
    assert!(last["error"].as_str().unwrap_or("").contains("tool not allowed"));

    server.abort();
}

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
    std::fs::write(workspace.join("large.txt"), format!("HEAD:{}:TAIL", "x".repeat(25_000)))
        .unwrap();

    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".rexos"),
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

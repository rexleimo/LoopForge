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

#[tokio::test]
async fn reserved_task_tools_post_claim_complete_and_publish_event() {
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
                        "tool_calls": [
                            {
                                "id": "call_1",
                                "type": "function",
                                "function": {
                                    "name": "task_post",
                                    "arguments": serde_json::to_string(&json!({
                                        "task_id": "t1",
                                        "title": "First task",
                                        "description": "Do thing",
                                        "assigned_to": null
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_2",
                                "type": "function",
                                "function": {
                                    "name": "task_list",
                                    "arguments": serde_json::to_string(&json!({ "status": "pending" })).unwrap()
                                }
                            },
                            {
                                "id": "call_3",
                                "type": "function",
                                "function": {
                                    "name": "task_claim",
                                    "arguments": serde_json::to_string(&json!({ "agent_id": "a1" })).unwrap()
                                }
                            },
                            {
                                "id": "call_4",
                                "type": "function",
                                "function": {
                                    "name": "task_complete",
                                    "arguments": serde_json::to_string(&json!({ "task_id": "t1", "result": "ok" })).unwrap()
                                }
                            },
                            {
                                "id": "call_5",
                                "type": "function",
                                "function": {
                                    "name": "event_publish",
                                    "arguments": serde_json::to_string(&json!({
                                        "event_type": "task_completed",
                                        "payload": { "task_id": "t1" }
                                    })).unwrap()
                                }
                            }
                        ]
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

    let home = tmp.path().join("home");
    let paths = rexos::paths::RexosPaths {
        base_dir: home.join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();

    let mut providers = BTreeMap::new();
    providers.insert(
        "mock".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url: format!("http://{addr}/v1"),
            api_key_env: "".to_string(),
            default_model: "x".to_string(),
            aws_bedrock: None,
        },
    );

    let cfg = rexos::config::RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers,
        router: rexos::config::RouterConfig::default(),
        security: Default::default(),
    };
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg).unwrap();
    let router = rexos::router::ModelRouter::new(rexos::config::RouterConfig {
        planning: rexos::config::RouteConfig {
            provider: "mock".to_string(),
            model: "x".to_string(),
        },
        coding: rexos::config::RouteConfig {
            provider: "mock".to_string(),
            model: "x".to_string(),
        },
        summary: rexos::config::RouteConfig {
            provider: "mock".to_string(),
            model: "x".to_string(),
        },
    });

    let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

    let out = agent
        .run_session(
            workspace,
            "s1",
            None,
            "exercise reserved task tools",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let tasks_idx = memory2
        .kv_get("rexos.tasks.index")
        .unwrap()
        .unwrap_or_default();
    assert!(
        tasks_idx.contains("t1"),
        "unexpected tasks index: {tasks_idx}"
    );

    let events = memory2.kv_get("rexos.events").unwrap().unwrap_or_default();
    assert!(
        events.contains("task_completed"),
        "unexpected events: {events}"
    );

    server.abort();
}

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
async fn reserved_agent_tools_spawn_list_find_kill_and_send() {
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
                                    "name": "agent_spawn",
                                    "arguments": serde_json::to_string(&json!({
                                        "agent_id": "a1",
                                        "name": "worker",
                                        "system_prompt": "You are worker."
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_2",
                                "type": "function",
                                "function": { "name": "agent_list", "arguments": "{}" }
                            },
                            {
                                "id": "call_3",
                                "type": "function",
                                "function": {
                                    "name": "agent_find",
                                    "arguments": serde_json::to_string(&json!({ "query": "work" })).unwrap()
                                }
                            },
                            {
                                "id": "call_4",
                                "type": "function",
                                "function": {
                                    "name": "agent_kill",
                                    "arguments": serde_json::to_string(&json!({ "agent_id": "a1" })).unwrap()
                                }
                            },
                            {
                                "id": "call_5",
                                "type": "function",
                                "function": {
                                    "name": "agent_send",
                                    "arguments": serde_json::to_string(&json!({ "agent_id": "a1", "message": "hello" })).unwrap()
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
        base_dir: home.join(".rexos"),
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
            "exercise reserved agent tools",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let list = memory2
        .kv_get("rexos.agents.index")
        .unwrap()
        .unwrap_or_default();
    assert!(list.contains("a1"), "unexpected agents index: {list}");

    let msgs = memory2.list_chat_messages("s1").unwrap();
    let list_msg = msgs
        .iter()
        .find(|m| {
            m.role == rexos::llm::openai_compat::Role::Tool
                && m.name.as_deref() == Some("agent_list")
        })
        .and_then(|m| m.content.clone())
        .expect("missing tool message: agent_list");
    let v: serde_json::Value = serde_json::from_str(&list_msg).expect("agent_list output is json");
    assert!(v.as_array().is_some(), "agent_list output was: {v}");

    let send_msg = msgs
        .iter()
        .find(|m| {
            m.role == rexos::llm::openai_compat::Role::Tool
                && m.name.as_deref() == Some("agent_send")
        })
        .and_then(|m| m.content.clone())
        .expect("missing tool message: agent_send");
    assert!(
        send_msg.to_lowercase().contains("killed"),
        "expected killed error, got: {send_msg}"
    );

    server.abort();
}

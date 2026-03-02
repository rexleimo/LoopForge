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

fn extract_hand_instance_id(payload: &serde_json::Value) -> Option<String> {
    let messages = payload.get("messages")?.as_array()?;
    for m in messages {
        if m.get("role").and_then(|v| v.as_str()) != Some("tool") {
            continue;
        }
        if m.get("name").and_then(|v| v.as_str()) != Some("hand_activate") {
            continue;
        }
        let content = m.get("content")?.as_str()?;
        let v: serde_json::Value = serde_json::from_str(content).ok()?;
        if let Some(id) = v.get("instance_id").and_then(|v| v.as_str()) {
            return Some(id.to_string());
        }
    }
    None
}

#[tokio::test]
async fn reserved_hand_tools_list_activate_status_and_deactivate() {
    async fn handler(
        State(state): State<TestState>,
        Json(payload): Json<serde_json::Value>,
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
                                "function": { "name": "hand_list", "arguments": "{}" }
                            },
                            {
                                "id": "call_2",
                                "type": "function",
                                "function": {
                                    "name": "hand_activate",
                                    "arguments": serde_json::to_string(&json!({
                                        "hand_id": "browser",
                                        "config": { "note": "test" }
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_3",
                                "type": "function",
                                "function": {
                                    "name": "hand_status",
                                    "arguments": serde_json::to_string(&json!({ "hand_id": "browser" })).unwrap()
                                }
                            }
                        ]
                    },
                    "finish_reason": "tool_calls"
                }]
            }));
        }

        if *calls == 2 {
            let instance_id = extract_hand_instance_id(&payload).unwrap_or_else(|| "missing".into());
            return Json(json!({
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": null,
                        "tool_calls": [
                            {
                                "id": "call_4",
                                "type": "function",
                                "function": {
                                    "name": "hand_deactivate",
                                    "arguments": serde_json::to_string(&json!({ "instance_id": instance_id })).unwrap()
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
            "exercise reserved hand tools",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let msgs = memory2.list_chat_messages("s1").unwrap();
    let list_msg = msgs
        .iter()
        .find(|m| {
            m.role == rexos::llm::openai_compat::Role::Tool
                && m.name.as_deref() == Some("hand_list")
        })
        .and_then(|m| m.content.clone())
        .expect("missing tool message: hand_list");
    let v: serde_json::Value = serde_json::from_str(&list_msg).expect("hand_list output is json");
    assert!(v.as_array().is_some(), "hand_list output was: {v}");

    let activate_msg = msgs
        .iter()
        .find(|m| {
            m.role == rexos::llm::openai_compat::Role::Tool
                && m.name.as_deref() == Some("hand_activate")
        })
        .and_then(|m| m.content.clone())
        .expect("missing tool message: hand_activate");
    let v: serde_json::Value =
        serde_json::from_str(&activate_msg).expect("hand_activate output is json");
    let instance_id = v
        .get("instance_id")
        .and_then(|v| v.as_str())
        .expect("hand_activate.instance_id");

    let idx_raw = memory2
        .kv_get("rexos.hands.instances.index")
        .unwrap()
        .unwrap_or_else(|| "[]".to_string());
    let idx: Vec<String> = serde_json::from_str(&idx_raw).unwrap_or_default();
    assert!(
        idx.iter().any(|v| v == instance_id),
        "unexpected hands index: {idx_raw}"
    );

    let agent_raw = memory2
        .kv_get(&format!("rexos.agents.{instance_id}"))
        .unwrap()
        .unwrap_or_default();
    assert!(
        agent_raw.contains("\"status\":\"killed\""),
        "expected killed agent record, got: {agent_raw}"
    );

    server.abort();
}

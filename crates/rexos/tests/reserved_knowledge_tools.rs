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
async fn reserved_knowledge_tools_add_entity_relation_and_query() {
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
                                    "name": "knowledge_add_entity",
                                    "arguments": serde_json::to_string(&json!({
                                        "id": "e1",
                                        "name": "LoopForge",
                                        "entity_type": "project",
                                        "properties": { "repo": "rexleimo/LoopForge" }
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_2",
                                "type": "function",
                                "function": {
                                    "name": "knowledge_add_entity",
                                    "arguments": serde_json::to_string(&json!({
                                        "id": "e2",
                                        "name": "Ollama",
                                        "entity_type": "tool",
                                        "properties": {}
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_3",
                                "type": "function",
                                "function": {
                                    "name": "knowledge_add_relation",
                                    "arguments": serde_json::to_string(&json!({
                                        "id": "r1",
                                        "source": "e1",
                                        "relation": "uses",
                                        "target": "e2",
                                        "properties": {}
                                    })).unwrap()
                                }
                            },
                            {
                                "id": "call_4",
                                "type": "function",
                                "function": {
                                    "name": "knowledge_query",
                                    "arguments": serde_json::to_string(&json!({ "query": "ollama" })).unwrap()
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
            "exercise reserved knowledge tools",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let msgs = memory2.list_chat_messages("s1").unwrap();
    let query_out = msgs
        .iter()
        .find(|m| {
            m.role == rexos::llm::openai_compat::Role::Tool
                && m.name.as_deref() == Some("knowledge_query")
        })
        .and_then(|m| m.content.clone())
        .expect("missing tool message: knowledge_query");

    let v: serde_json::Value = serde_json::from_str(&query_out).expect("knowledge_query is json");
    assert!(
        v.get("entities").and_then(|v| v.as_array()).is_some(),
        "{v}"
    );
    assert!(
        v["entities"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e.get("id").and_then(|v| v.as_str()) == Some("e2")),
        "{v}"
    );
    assert!(
        v["relations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.get("id").and_then(|v| v.as_str()) == Some("r1")),
        "{v}"
    );

    server.abort();
}

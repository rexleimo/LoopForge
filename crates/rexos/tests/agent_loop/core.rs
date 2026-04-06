use std::collections::BTreeMap;

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use super::common::{
    default_agent, mcp_config_json, workspace_and_paths, write_mcp_stub, TestState,
};

#[tokio::test]
async fn agent_loop_executes_tool_calls_and_persists_history() {
    async fn handler(
        State(state): State<TestState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        *state.last_request.lock().unwrap() = Some(payload);
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
                                "name": "fs_write",
                                "arguments": "{\"path\":\"hello.txt\",\"content\":\"hi\"}"
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
                "message": {
                    "role": "assistant",
                    "content": "done"
                },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let (workspace, paths) = workspace_and_paths(&tmp);
    let agent = default_agent(&paths, format!("http://{addr}/v1"));

    let out = agent
        .run_session(
            workspace.clone(),
            "s1",
            None,
            "write hello file",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    assert_eq!(
        std::fs::read_to_string(workspace.join("hello.txt")).unwrap(),
        "hi"
    );

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let msgs = memory2.list_chat_messages("s1").unwrap();
    assert!(msgs.len() >= 4);

    let last_req = state.last_request.lock().unwrap().clone().unwrap();
    let roles: Vec<String> = last_req["messages"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|m| {
            m.get("role")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string())
        })
        .collect();
    assert!(roles.contains(&"tool".to_string()));
    assert_eq!(
        last_req.get("temperature").and_then(|v| v.as_f64()),
        Some(0.0)
    );

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_mcp_tool_calls_when_session_config_is_set() {
    async fn handler(
        State(state): State<TestState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        *state.last_request.lock().unwrap() = Some(payload);
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
                                "name": "mcp_stub__echo",
                                "arguments": "{\"text\":\"yo\"}"
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
                "message": {
                    "role": "assistant",
                    "content": "done"
                },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let (workspace, paths) = workspace_and_paths(&tmp);
    let mcp_stub = write_mcp_stub(&workspace);

    let agent = default_agent(&paths, format!("http://{addr}/v1"));
    agent
        .set_session_mcp_config("s1", mcp_config_json(&mcp_stub))
        .unwrap();

    let out = agent
        .run_session(
            workspace.clone(),
            "s1",
            None,
            "call mcp",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let last_req = state.last_request.lock().unwrap().clone().unwrap();
    let tool_names: Vec<&str> = last_req["tools"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|t| t.get("function")?.get("name")?.as_str())
        .collect();
    assert!(tool_names.contains(&"mcp_stub__echo"), "{tool_names:?}");

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_memory_store_and_persists_kv() {
    async fn handler(
        State(state): State<TestState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        *state.last_request.lock().unwrap() = Some(payload);
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
                                "name": "memory_store",
                                "arguments": "{\"key\":\"k1\",\"value\":\"v1\"}"
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
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let (workspace, paths) = workspace_and_paths(&tmp);
    let agent = default_agent(&paths, format!("http://{addr}/v1"));

    let out = agent
        .run_session(
            workspace.clone(),
            "s1",
            None,
            "store something",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let v = memory2.kv_get("k1").unwrap();
    assert_eq!(v.as_deref(), Some("v1"));

    server.abort();
}

#[tokio::test]
async fn agent_loop_truncates_large_tool_results_with_head_and_tail() {
    async fn handler(
        State(state): State<TestState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        *state.last_request.lock().unwrap() = Some(payload);
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
                "message": {
                    "role": "assistant",
                    "content": "done"
                },
                "finish_reason": "stop"
            }]
        }))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/chat/completions", post(handler))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let (workspace, paths) = workspace_and_paths(&tmp);
    let large_content = format!("HEAD_MARKER:{}:TAIL_MARKER", "x".repeat(22_000));
    std::fs::write(workspace.join("large.txt"), large_content).unwrap();

    let agent = default_agent(&paths, format!("http://{addr}/v1"));
    let out = agent
        .run_session(
            workspace.clone(),
            "s1",
            None,
            "read large file",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let last_req = state.last_request.lock().unwrap().clone().unwrap();
    let tool_message = last_req["messages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m.get("role").and_then(|v| v.as_str()) == Some("tool"))
        .cloned()
        .expect("tool message should exist in second LLM request");
    let tool_content = tool_message
        .get("content")
        .and_then(|v| v.as_str())
        .expect("tool content should be string");
    assert!(
        tool_content.contains("HEAD_MARKER"),
        "expected truncated output to preserve head marker\ncontent:\n{tool_content}"
    );
    assert!(
        tool_content.contains("TAIL_MARKER"),
        "expected truncated output to preserve tail marker\ncontent:\n{tool_content}"
    );
    assert!(
        tool_content.contains("omitted"),
        "expected omission marker in truncated output\ncontent:\n{tool_content}"
    );

    server.abort();
}

#[tokio::test]
async fn agent_loop_should_route_by_provider_name() {
    async fn handler_p1() -> Json<serde_json::Value> {
        Json(json!({
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "p1" },
                "finish_reason": "stop"
            }]
        }))
    }

    async fn handler_p2() -> Json<serde_json::Value> {
        Json(json!({
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "p2" },
                "finish_reason": "stop"
            }]
        }))
    }

    let app1 = Router::new().route("/v1/chat/completions", post(handler_p1));
    let listener1 = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr1 = listener1.local_addr().unwrap();
    let server1 = tokio::spawn(async move {
        axum::serve(listener1, app1).await.unwrap();
    });

    let app2 = Router::new().route("/v1/chat/completions", post(handler_p2));
    let listener2 = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr2 = listener2.local_addr().unwrap();
    let server2 = tokio::spawn(async move {
        axum::serve(listener2, app2).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let (workspace, paths) = workspace_and_paths(&tmp);
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();

    let mut providers = BTreeMap::new();
    providers.insert(
        "p1".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url: format!("http://{addr1}/v1"),
            api_key_env: "".to_string(),
            default_model: "x".to_string(),
            aws_bedrock: None,
        },
    );
    providers.insert(
        "p2".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url: format!("http://{addr2}/v1"),
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
            provider: "p1".to_string(),
            model: "x".to_string(),
        },
        coding: rexos::config::RouteConfig {
            provider: "p1".to_string(),
            model: "x".to_string(),
        },
        summary: rexos::config::RouteConfig {
            provider: "p1".to_string(),
            model: "x".to_string(),
        },
    });

    let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

    let out = agent
        .run_session(
            workspace,
            "s-route",
            None,
            "hello",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();

    assert_eq!(out, "p1");

    server1.abort();
    server2.abort();
}

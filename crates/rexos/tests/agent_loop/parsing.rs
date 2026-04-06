use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use super::common::{default_agent, workspace_and_paths, TestState};

#[tokio::test]
async fn agent_loop_executes_tool_calls_from_json_content_fallback() {
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
                        "content": "[{\"name\":\"fs_write\",\"arguments\":{\"path\":\"hello.txt\",\"content\":\"hi\"}}]"
                    },
                    "finish_reason": "stop"
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
            "s-json-fallback",
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

    server.abort();
}

#[tokio::test]
async fn agent_loop_unwraps_wrapped_tool_arguments() {
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
                                "arguments": "{\"function\":\"fs_write\",\"arguments\":{\"path\":\"hello.txt\",\"content\":\"hi\"}}"
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
            "s-unwrap-args",
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

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_tool_calls_from_embedded_json_snippets() {
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
                        "content": "Call this tool:\n{\"name\":\"fs_write\",\"arguments\":{\"path\":\"hello.txt\",\"content\":\"hi\"}}"
                    },
                    "finish_reason": "stop"
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
            "s-json-snippets",
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

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_tool_calls_from_tagged_tool_call_json() {
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
                        "content": "[tool_call]\n{\n  \"name\": \"fs_write\",\n  \"args\": {\"path\": \"hello.txt\", \"content\": \"hi\"}\n}"
                    },
                    "finish_reason": "stop"
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
            "s-tagged-tool-call",
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

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_tool_calls_from_flattened_call_objects() {
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
                        "content": "[{\"name\":\"fs_write\",\"path\":\"hello.txt\",\"content\":\"hi\"}]"
                    },
                    "finish_reason": "stop"
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
            "s-flat-call",
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

    server.abort();
}

#[tokio::test]
async fn agent_loop_executes_tool_calls_from_function_name_json_objects() {
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
                        "content": "{\n  \"function_name\": \"file_write\",\n  \"arguments\": {\n    \"path\": \"notes/workspace-brief.md\",\n    \"content\": \"brief\"\n  }\n}"
                    },
                    "finish_reason": "stop"
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
            "s-function-name-call",
            None,
            "write onboarding brief",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    assert_eq!(
        std::fs::read_to_string(workspace.join("notes/workspace-brief.md")).unwrap(),
        "brief"
    );

    server.abort();
}

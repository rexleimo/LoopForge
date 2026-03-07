use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use rexos::llm::driver::LlmDriver;

#[derive(Clone, Default)]
struct TestState {
    captured: Arc<Mutex<Option<serde_json::Value>>>,
}

#[tokio::test]
async fn minimax_driver_maps_messages_tools_and_tool_calls() {
    async fn handler(
        State(state): State<TestState>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        let auth = headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_eq!(auth, "Bearer k");

        *state.captured.lock().unwrap() = Some(payload);

        Ok(Json(json!({
            "choices": [{
                "index": 0,
                "finish_reason": "tool_calls",
                "message": {
                    "role": "assistant",
                    "content": "",
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": { "name": "fs_read", "arguments": "{\"path\":\"README.md\"}" }
                    }]
                }
            }]
        })))
    }

    let state = TestState::default();
    let app = Router::new()
        .route("/v1/text/chatcompletion_v2", post(handler))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let driver =
        rexos::llm::minimax::MiniMaxDriver::new(format!("http://{addr}/v1"), Some("k".to_string()))
            .unwrap();

    let req = rexos::llm::openai_compat::ChatCompletionRequest {
        model: "MiniMax-M2.5".to_string(),
        messages: vec![
            rexos::llm::openai_compat::ChatMessage {
                role: rexos::llm::openai_compat::Role::System,
                content: Some("sys".to_string()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            },
            rexos::llm::openai_compat::ChatMessage {
                role: rexos::llm::openai_compat::Role::User,
                content: Some("read it".to_string()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            },
        ],
        tools: vec![rexos::llm::openai_compat::ToolDefinition {
            kind: "function".to_string(),
            function: rexos::llm::openai_compat::ToolFunctionDefinition {
                name: "fs_read".to_string(),
                description: "Read file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"],
                    "additionalProperties": false
                }),
            },
        }],
        temperature: Some(0.7),
    };

    let msg = driver.chat(req).await.unwrap();
    assert_eq!(msg.role, rexos::llm::openai_compat::Role::Assistant);
    let calls = msg.tool_calls.unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].function.name, "fs_read");

    let captured = state.captured.lock().unwrap().clone().unwrap();
    assert_eq!(captured["model"], "MiniMax-M2.5");
    assert_eq!(captured["stream"], false);
    assert_eq!(captured["temperature"], 0.7);
    assert_eq!(captured["tools"][0]["function"]["name"], "fs_read");
    assert_eq!(captured["tool_choice"], "auto");
    assert_eq!(captured["messages"][0]["role"], "system");

    server.abort();
}

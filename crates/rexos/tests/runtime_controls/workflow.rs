use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;

use super::common::{test_agent, TestState};

#[tokio::test]
async fn workflow_run_persists_state_and_executes_steps() {
    async fn handler(
        State(state): State<TestState>,
        Json(_payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let mut calls = state.calls.lock().unwrap();
        *calls += 1;
        if *calls == 1 {
            let args = json!({
                "workflow_id": "wf-demo",
                "name": "demo",
                "steps": [
                    {
                        "name": "write note",
                        "tool": "fs_write",
                        "arguments": { "path": "workflow-note.txt", "content": "hello workflow" }
                    }
                ]
            });
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
                                "name": "workflow_run",
                                "arguments": serde_json::to_string(&args).unwrap()
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
            workspace.clone(),
            "s-workflow",
            None,
            "run workflow",
            rexos::router::TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");
    assert_eq!(
        std::fs::read_to_string(workspace.join("workflow-note.txt")).unwrap(),
        "hello workflow"
    );

    let state_path = workspace.join(".loopforge/workflows/wf-demo.json");
    let state_raw = std::fs::read_to_string(&state_path).unwrap();
    let state_json: serde_json::Value = serde_json::from_str(&state_raw).unwrap();
    assert_eq!(state_json["status"], "completed");
    assert_eq!(state_json["steps"][0]["status"], "succeeded");

    server.abort();
}

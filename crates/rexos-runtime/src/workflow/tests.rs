use serde_json::json;

use crate::records::WorkflowStepToolArgs;

use super::execution::serialize_workflow_step_arguments;
use super::state::build_workflow_state;

#[test]
fn workflow_state_starts_with_pending_steps() {
    let steps = vec![
        WorkflowStepToolArgs {
            tool: "fs_write".to_string(),
            arguments: json!({ "path": "note.txt", "content": "hello" }),
            name: Some("write note".to_string()),
            approval_required: Some(false),
        },
        WorkflowStepToolArgs {
            tool: "fs_read".to_string(),
            arguments: json!({ "path": "note.txt" }),
            name: Some("read note".to_string()),
            approval_required: Some(true),
        },
    ];

    let state = build_workflow_state(
        "wf-demo",
        Some("demo".to_string()),
        "session-demo",
        &steps,
        123,
    );

    assert_eq!(state.workflow_id, "wf-demo");
    assert_eq!(state.name.as_deref(), Some("demo"));
    assert_eq!(state.session_id, "session-demo");
    assert_eq!(state.status, "running");
    assert_eq!(state.created_at, 123);
    assert_eq!(state.updated_at, 123);
    assert_eq!(state.completed_at, None);
    assert_eq!(state.steps.len(), 2);
    assert_eq!(state.steps[0].index, 0);
    assert_eq!(state.steps[0].name.as_deref(), Some("write note"));
    assert_eq!(state.steps[0].tool, "fs_write");
    assert_eq!(
        state.steps[0].arguments,
        json!({ "path": "note.txt", "content": "hello" })
    );
    assert_eq!(state.steps[0].status, "pending");
    assert_eq!(state.steps[0].output, None);
    assert_eq!(state.steps[0].error, None);
    assert_eq!(state.steps[0].started_at, None);
    assert_eq!(state.steps[0].completed_at, None);
    assert_eq!(state.steps[1].index, 1);
    assert_eq!(state.steps[1].name.as_deref(), Some("read note"));
    assert_eq!(state.steps[1].tool, "fs_read");
    assert_eq!(state.steps[1].status, "pending");
}

#[test]
fn workflow_step_arguments_serialize_null_as_empty_object() {
    assert_eq!(
        serialize_workflow_step_arguments(&serde_json::Value::Null).unwrap(),
        "{}"
    );
    assert_eq!(
        serialize_workflow_step_arguments(&json!({ "path": "note.txt" })).unwrap(),
        r#"{"path":"note.txt"}"#
    );
}

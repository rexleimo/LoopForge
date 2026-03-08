use crate::records::{AcpEventRecord, WorkflowRunStateRecord};
use crate::tool_calls::truncate_tool_result_with_flag;
use crate::AgentRuntime;

pub(super) fn emit_workflow_started(
    runtime: &AgentRuntime,
    session_id: &str,
    workflow_id: &str,
    step_count: usize,
) {
    let _ = runtime.append_acp_event(AcpEventRecord {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: Some(session_id.to_string()),
        event_type: "workflow.started".to_string(),
        payload: serde_json::json!({
            "workflow_id": workflow_id,
            "steps": step_count,
        }),
        created_at: AgentRuntime::now_epoch_seconds(),
    });
}

pub(super) fn record_workflow_step_success(
    runtime: &AgentRuntime,
    state: &mut WorkflowRunStateRecord,
    session_id: &str,
    workflow_id: &str,
    idx: usize,
    tool_name: &str,
    completed_at: i64,
    output: String,
) {
    let step = &mut state.steps[idx];
    step.completed_at = Some(completed_at);
    let (output, _) = truncate_tool_result_with_flag(output, 4_000);
    step.status = "succeeded".to_string();
    step.output = Some(output);
    step.error = None;

    let _ = runtime.append_acp_event(AcpEventRecord {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: Some(session_id.to_string()),
        event_type: "workflow.step_succeeded".to_string(),
        payload: serde_json::json!({
            "workflow_id": workflow_id,
            "step_index": idx,
            "tool": tool_name,
        }),
        created_at: completed_at,
    });
}

pub(super) fn record_workflow_step_failure(
    runtime: &AgentRuntime,
    state: &mut WorkflowRunStateRecord,
    session_id: &str,
    workflow_id: &str,
    idx: usize,
    tool_name: &str,
    completed_at: i64,
    error: &str,
) {
    let step = &mut state.steps[idx];
    step.completed_at = Some(completed_at);
    step.status = "failed".to_string();
    step.output = None;
    step.error = Some(error.to_string());
    state.status = "failed".to_string();

    let _ = runtime.append_acp_event(AcpEventRecord {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: Some(session_id.to_string()),
        event_type: "workflow.step_failed".to_string(),
        payload: serde_json::json!({
            "workflow_id": workflow_id,
            "step_index": idx,
            "tool": tool_name,
            "error": error,
        }),
        created_at: completed_at,
    });
}

pub(super) fn emit_workflow_finished(
    runtime: &AgentRuntime,
    session_id: &str,
    workflow_id: &str,
    status: &str,
    failed_steps: usize,
) {
    let _ = runtime.append_acp_event(AcpEventRecord {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: Some(session_id.to_string()),
        event_type: if status == "completed" {
            "workflow.completed".to_string()
        } else {
            "workflow.failed".to_string()
        },
        payload: serde_json::json!({
            "workflow_id": workflow_id,
            "status": status,
            "failed_steps": failed_steps,
        }),
        created_at: AgentRuntime::now_epoch_seconds(),
    });
}

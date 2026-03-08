mod events;
mod result;
mod steps;

use std::path::Path;

use rexos_tools::Toolset;

use crate::records::{WorkflowRunStateRecord, WorkflowStepToolArgs};
use crate::AgentRuntime;

pub(super) fn serialize_workflow_step_arguments(
    arguments: &serde_json::Value,
) -> anyhow::Result<String> {
    steps::serialize_workflow_step_arguments(arguments)
}

pub(super) async fn execute_workflow_step(
    runtime: &AgentRuntime,
    tools: &Toolset,
    session_id: &str,
    workflow_id: &str,
    idx: usize,
    step: &WorkflowStepToolArgs,
    args_json: &str,
) -> anyhow::Result<String> {
    steps::execute_workflow_step(
        runtime,
        tools,
        session_id,
        workflow_id,
        idx,
        step,
        args_json,
    )
    .await
}

pub(super) fn emit_workflow_started(
    runtime: &AgentRuntime,
    session_id: &str,
    workflow_id: &str,
    step_count: usize,
) {
    events::emit_workflow_started(runtime, session_id, workflow_id, step_count)
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
    events::record_workflow_step_success(
        runtime,
        state,
        session_id,
        workflow_id,
        idx,
        tool_name,
        completed_at,
        output,
    )
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
    events::record_workflow_step_failure(
        runtime,
        state,
        session_id,
        workflow_id,
        idx,
        tool_name,
        completed_at,
        error,
    )
}

pub(super) fn emit_workflow_finished(
    runtime: &AgentRuntime,
    session_id: &str,
    workflow_id: &str,
    status: &str,
    failed_steps: usize,
) {
    events::emit_workflow_finished(runtime, session_id, workflow_id, status, failed_steps)
}

pub(super) fn workflow_result_json(
    state: &WorkflowRunStateRecord,
    failed_steps: usize,
    state_path: &Path,
) -> String {
    result::workflow_result_json(state, failed_steps, state_path)
}

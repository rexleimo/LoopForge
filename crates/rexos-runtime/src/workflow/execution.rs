mod events;
mod result;
mod steps;

use std::path::Path;

use rexos_tools::Toolset;

use crate::records::{WorkflowRunStateRecord, WorkflowStepToolArgs};
use crate::AgentRuntime;
pub(super) use events::WorkflowStepEvent;

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
    step_event: WorkflowStepEvent<'_>,
    output: String,
) {
    events::record_workflow_step_success(runtime, state, step_event, output)
}

pub(super) fn record_workflow_step_failure(
    runtime: &AgentRuntime,
    state: &mut WorkflowRunStateRecord,
    step_event: WorkflowStepEvent<'_>,
    error: &str,
) {
    events::record_workflow_step_failure(runtime, state, step_event, error)
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

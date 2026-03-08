use anyhow::{bail, Context};
use rexos_tools::Toolset;

use crate::records::{AcpEventRecord, WorkflowStepToolArgs};
use crate::{is_runtime_managed_tool, AgentRuntime};

pub(super) fn serialize_workflow_step_arguments(
    arguments: &serde_json::Value,
) -> anyhow::Result<String> {
    if arguments.is_null() {
        return Ok("{}".to_string());
    }
    serde_json::to_string(arguments).context("serialize workflow step arguments")
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
    if is_runtime_managed_tool(&step.tool) {
        bail!(
            "workflow step tool `{}` is runtime-managed and not supported in workflow_run yet",
            step.tool
        );
    }

    if let Some(warning) = runtime.evaluate_tool_approval(
        session_id,
        &step.tool,
        args_json,
        step.approval_required.unwrap_or(false),
    )? {
        let _ = runtime.append_acp_event(AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "approval.warn".to_string(),
            payload: serde_json::json!({
                "tool": step.tool,
                "message": warning,
                "workflow_id": workflow_id,
                "step_index": idx,
            }),
            created_at: AgentRuntime::now_epoch_seconds(),
        });
    }

    tools
        .call(&step.tool, args_json)
        .await
        .with_context(|| format!("workflow step {} ({})", idx, step.tool))
}

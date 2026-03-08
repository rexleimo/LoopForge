use std::path::Path;

use anyhow::Context;

use crate::records::{WorkflowRunStateRecord, WorkflowStepStateRecord, WorkflowStepToolArgs};
use crate::AgentRuntime;

pub(super) fn build_workflow_state(
    workflow_id: &str,
    name: Option<String>,
    session_id: &str,
    steps: &[WorkflowStepToolArgs],
    now: i64,
) -> WorkflowRunStateRecord {
    WorkflowRunStateRecord {
        workflow_id: workflow_id.to_string(),
        name,
        session_id: session_id.to_string(),
        status: "running".to_string(),
        created_at: now,
        updated_at: now,
        completed_at: None,
        steps: steps
            .iter()
            .enumerate()
            .map(|(idx, step)| WorkflowStepStateRecord {
                index: idx,
                name: step.name.clone(),
                tool: step.tool.clone(),
                arguments: step.arguments.clone(),
                status: "pending".to_string(),
                output: None,
                error: None,
                started_at: None,
                completed_at: None,
            })
            .collect(),
    }
}

pub(super) fn mark_workflow_step_running(
    state: &mut WorkflowRunStateRecord,
    idx: usize,
    started_at: i64,
) {
    let step = &mut state.steps[idx];
    step.status = "running".to_string();
    step.started_at = Some(started_at);
    step.completed_at = None;
    step.error = None;
    state.updated_at = started_at;
}

pub(super) fn finalize_workflow_state(state: &mut WorkflowRunStateRecord, completed_at: i64) {
    if state.status != "failed" {
        state.status = "completed".to_string();
    }
    state.completed_at = Some(completed_at);
    state.updated_at = completed_at;
}

impl AgentRuntime {
    pub(super) fn write_workflow_state(
        &self,
        path: &Path,
        state: &WorkflowRunStateRecord,
    ) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create workflow dir {}", parent.display()))?;
        }
        let raw = serde_json::to_string_pretty(state).context("serialize workflow state")?;
        std::fs::write(path, raw)
            .with_context(|| format!("write workflow state {}", path.display()))
    }
}

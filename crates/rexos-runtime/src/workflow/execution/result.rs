use std::path::Path;

use crate::records::WorkflowRunStateRecord;

pub(super) fn workflow_result_json(
    state: &WorkflowRunStateRecord,
    failed_steps: usize,
    state_path: &Path,
) -> String {
    serde_json::json!({
        "workflow_id": state.workflow_id,
        "name": state.name,
        "status": state.status,
        "failed_steps": failed_steps,
        "saved_to": state_path.display().to_string(),
    })
    .to_string()
}

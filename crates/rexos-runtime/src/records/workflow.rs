#[derive(Debug, serde::Deserialize)]
pub(crate) struct WorkflowRunToolArgs {
    #[serde(default)]
    pub(crate) workflow_id: Option<String>,
    #[serde(default)]
    pub(crate) name: Option<String>,
    pub(crate) steps: Vec<WorkflowStepToolArgs>,
    #[serde(default)]
    pub(crate) continue_on_error: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct WorkflowStepToolArgs {
    pub(crate) tool: String,
    #[serde(default)]
    pub(crate) arguments: serde_json::Value,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) approval_required: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct WorkflowRunStateRecord {
    pub(crate) workflow_id: String,
    #[serde(default)]
    pub(crate) name: Option<String>,
    pub(crate) session_id: String,
    pub(crate) status: String,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
    #[serde(default)]
    pub(crate) completed_at: Option<i64>,
    pub(crate) steps: Vec<WorkflowStepStateRecord>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct WorkflowStepStateRecord {
    pub(crate) index: usize,
    #[serde(default)]
    pub(crate) name: Option<String>,
    pub(crate) tool: String,
    pub(crate) arguments: serde_json::Value,
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) output: Option<String>,
    #[serde(default)]
    pub(crate) error: Option<String>,
    #[serde(default)]
    pub(crate) started_at: Option<i64>,
    #[serde(default)]
    pub(crate) completed_at: Option<i64>,
}

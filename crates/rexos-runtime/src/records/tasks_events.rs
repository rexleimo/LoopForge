#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TaskStatus {
    Pending,
    Claimed,
    Completed,
}

impl TaskStatus {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Claimed => "claimed",
            TaskStatus::Completed => "completed",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct TaskRecord {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) assigned_to: Option<String>,
    pub(crate) status: TaskStatus,
    pub(crate) claimed_by: Option<String>,
    pub(crate) result: Option<String>,
    pub(crate) created_at: i64,
    pub(crate) claimed_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TaskPostToolArgs {
    #[serde(default)]
    pub(crate) task_id: Option<String>,
    pub(crate) title: String,
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) assigned_to: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TaskListToolArgs {
    #[serde(default)]
    pub(crate) status: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TaskClaimToolArgs {
    #[serde(default)]
    pub(crate) agent_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TaskCompleteToolArgs {
    pub(crate) task_id: String,
    pub(crate) result: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct EventRecord {
    pub(crate) id: String,
    pub(crate) event_type: String,
    pub(crate) payload: serde_json::Value,
    pub(crate) created_at: i64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct EventPublishToolArgs {
    pub(crate) event_type: String,
    #[serde(default)]
    pub(crate) payload: Option<serde_json::Value>,
}

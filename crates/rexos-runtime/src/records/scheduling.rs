#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ScheduleRecord {
    pub(crate) id: String,
    pub(crate) description: String,
    pub(crate) schedule: String,
    pub(crate) agent_id: Option<String>,
    pub(crate) created_at: i64,
    pub(crate) enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ScheduleCreateToolArgs {
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) description: String,
    pub(crate) schedule: String,
    #[serde(default)]
    pub(crate) agent_id: Option<String>,
    #[serde(default)]
    pub(crate) agent: Option<String>,
    #[serde(default)]
    pub(crate) enabled: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ScheduleDeleteToolArgs {
    pub(crate) id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct CronJobRecord {
    pub(crate) job_id: String,
    pub(crate) name: String,
    pub(crate) schedule: serde_json::Value,
    pub(crate) action: serde_json::Value,
    #[serde(default)]
    pub(crate) delivery: Option<serde_json::Value>,
    pub(crate) one_shot: bool,
    pub(crate) created_at: i64,
    pub(crate) enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CronCreateToolArgs {
    #[serde(default)]
    #[serde(alias = "id")]
    pub(crate) job_id: Option<String>,
    pub(crate) name: String,
    pub(crate) schedule: serde_json::Value,
    pub(crate) action: serde_json::Value,
    #[serde(default)]
    pub(crate) delivery: Option<serde_json::Value>,
    #[serde(default)]
    pub(crate) one_shot: Option<bool>,
    #[serde(default)]
    pub(crate) enabled: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CronCancelToolArgs {
    #[serde(alias = "id")]
    pub(crate) job_id: String,
}

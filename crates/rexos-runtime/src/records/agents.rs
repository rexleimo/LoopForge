#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AgentStatus {
    Running,
    Killed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentRecord {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) system_prompt: Option<String>,
    pub(crate) status: AgentStatus,
    pub(crate) created_at: i64,
    pub(crate) killed_at: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct AgentSpawnToolArgs {
    #[serde(default)]
    pub(crate) agent_id: Option<String>,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) system_prompt: Option<String>,
    #[serde(default)]
    pub(crate) manifest_toml: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct AgentSendToolArgs {
    pub(crate) agent_id: String,
    pub(crate) message: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct AgentKillToolArgs {
    pub(crate) agent_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct AgentFindToolArgs {
    pub(crate) query: String,
}

#[derive(Debug, Clone)]
pub(crate) struct HandDef {
    pub(crate) id: &'static str,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) system_prompt: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HandInstanceStatus {
    Active,
    Deactivated,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct HandInstanceRecord {
    pub(crate) instance_id: String,
    pub(crate) hand_id: String,
    pub(crate) agent_id: String,
    pub(crate) status: HandInstanceStatus,
    pub(crate) created_at: i64,
    #[serde(default)]
    pub(crate) deactivated_at: Option<i64>,
    #[serde(default)]
    pub(crate) config: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HandActivateToolArgs {
    pub(crate) hand_id: String,
    #[serde(default)]
    pub(crate) config: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HandStatusToolArgs {
    pub(crate) hand_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HandDeactivateToolArgs {
    pub(crate) instance_id: String,
}

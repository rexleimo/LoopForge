#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ToolAuditRecord {
    pub(crate) session_id: String,
    pub(crate) tool_name: String,
    pub(crate) success: bool,
    pub(crate) duration_ms: u64,
    pub(crate) truncated: bool,
    #[serde(default)]
    pub(crate) error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) leak_guard: Option<crate::leak_guard::LeakGuardAudit>,
    pub(crate) created_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SessionSkillPolicy {
    #[serde(default)]
    pub allowlist: Vec<String>,
    #[serde(default)]
    pub require_approval: bool,
    #[serde(default = "default_auto_approve_readonly")]
    pub auto_approve_readonly: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct SkillAuditRecord {
    pub(crate) session_id: String,
    pub(crate) skill_name: String,
    pub(crate) success: bool,
    #[serde(default)]
    pub(crate) permissions: Vec<String>,
    #[serde(default)]
    pub(crate) error: Option<String>,
    pub(crate) created_at: i64,
}

fn default_auto_approve_readonly() -> bool {
    true
}

use anyhow::bail;

use crate::approval::permissions::tool_approval_is_granted;
use crate::{AcpEventRecord, AgentRuntime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ApprovalMode {
    Off,
    Warn,
    Enforce,
}

impl ApprovalMode {
    pub(crate) fn from_env() -> Self {
        let raw = std::env::var("LOOPFORGE_APPROVAL_MODE")
            .unwrap_or_else(|_| "off".to_string())
            .to_lowercase();
        match raw.as_str() {
            "warn" => Self::Warn,
            "enforce" => Self::Enforce,
            _ => Self::Off,
        }
    }
}

pub(crate) fn tool_requires_approval(
    name: &str,
    arguments_json: &str,
    explicit_gate: bool,
) -> bool {
    if explicit_gate {
        return true;
    }

    match name {
        "shell" | "docker_exec" | "process_start" => true,
        "web_fetch" | "browser_navigate" => json_bool_field(arguments_json, "allow_private"),
        _ => false,
    }
}

fn json_bool_field(arguments_json: &str, field: &str) -> bool {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(arguments_json) else {
        return false;
    };
    value
        .get(field)
        .and_then(|field_value| field_value.as_bool())
        .unwrap_or(false)
}

impl AgentRuntime {
    pub(crate) fn evaluate_tool_approval(
        &self,
        session_id: &str,
        tool_name: &str,
        arguments_json: &str,
        explicit_gate: bool,
    ) -> anyhow::Result<Option<String>> {
        let mode = ApprovalMode::from_env();
        if mode == ApprovalMode::Off {
            return Ok(None);
        }
        if !tool_requires_approval(tool_name, arguments_json, explicit_gate) {
            return Ok(None);
        }
        if tool_approval_is_granted(tool_name) {
            return Ok(None);
        }

        let msg = format!(
            "approval required for dangerous tool `{tool_name}` (set LOOPFORGE_APPROVAL_ALLOW={tool_name} or all)"
        );
        match mode {
            ApprovalMode::Warn => Ok(Some(msg)),
            ApprovalMode::Enforce => {
                let _ = self.append_acp_event(AcpEventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    session_id: Some(session_id.to_string()),
                    event_type: "approval.blocked".to_string(),
                    payload: serde_json::json!({
                        "tool": tool_name,
                        "message": msg,
                    }),
                    created_at: Self::now_epoch_seconds(),
                });
                bail!("{msg}")
            }
            ApprovalMode::Off => Ok(None),
        }
    }
}

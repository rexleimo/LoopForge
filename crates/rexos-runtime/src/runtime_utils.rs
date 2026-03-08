use std::path::{Path, PathBuf};

use crate::leak_guard::LeakGuardAudit;

pub(crate) fn tool_event_payload(
    tool_name: &str,
    truncated: Option<bool>,
    error: Option<&str>,
    reason: Option<&str>,
    leak_guard: Option<&LeakGuardAudit>,
) -> serde_json::Value {
    let mut payload = serde_json::Map::new();
    payload.insert(
        "tool".to_string(),
        serde_json::Value::String(tool_name.to_string()),
    );
    if let Some(truncated) = truncated {
        payload.insert("truncated".to_string(), serde_json::Value::Bool(truncated));
    }
    if let Some(error) = error {
        payload.insert(
            "error".to_string(),
            serde_json::Value::String(error.to_string()),
        );
    }
    if let Some(reason) = reason {
        payload.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
    }
    if let Some(leak_guard) = leak_guard {
        if let Ok(value) = serde_json::to_value(leak_guard) {
            payload.insert("leak_guard".to_string(), value);
        }
    }
    serde_json::Value::Object(payload)
}

pub(crate) fn workflow_state_path(workspace_root: &Path, workflow_id: &str) -> PathBuf {
    workspace_root
        .join(".loopforge")
        .join("workflows")
        .join(format!("{workflow_id}.json"))
}

pub(crate) fn is_runtime_managed_tool(name: &str) -> bool {
    matches!(
        name,
        "memory_store"
            | "memory_recall"
            | "agent_send"
            | "agent_spawn"
            | "agent_list"
            | "agent_kill"
            | "agent_find"
            | "hand_list"
            | "hand_activate"
            | "hand_status"
            | "hand_deactivate"
            | "task_post"
            | "task_claim"
            | "task_complete"
            | "task_list"
            | "event_publish"
            | "schedule_create"
            | "schedule_list"
            | "schedule_delete"
            | "knowledge_add_entity"
            | "knowledge_add_relation"
            | "knowledge_query"
            | "cron_create"
            | "cron_list"
            | "cron_cancel"
            | "channel_send"
            | "workflow_run"
    )
}

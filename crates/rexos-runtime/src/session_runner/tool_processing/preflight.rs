use std::collections::HashSet;
use std::time::Instant;

use anyhow::bail;
use rexos_llm::openai_compat::ToolCall;

use crate::records::{AcpEventRecord, ToolAuditRecord};
use crate::AgentRuntime;

pub(super) fn enforce_session_tool_whitelist(
    runtime: &AgentRuntime,
    session_id: &str,
    allowed_lookup: Option<&HashSet<String>>,
    call: &ToolCall,
    started_at: &Instant,
) -> anyhow::Result<()> {
    if let Some(allowed) = allowed_lookup {
        if !allowed.contains(call.function.name.as_str()) {
            let err = format!("tool not allowed for this session: {}", call.function.name);
            let _ = runtime.append_acp_event(AcpEventRecord {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: Some(session_id.to_string()),
                event_type: "tool.blocked".to_string(),
                payload: serde_json::json!({
                    "tool": call.function.name.clone(),
                    "reason": "session_whitelist",
                }),
                created_at: AgentRuntime::now_epoch_seconds(),
            });
            let _ = runtime.append_tool_audit(ToolAuditRecord {
                session_id: session_id.to_string(),
                tool_name: call.function.name.clone(),
                success: false,
                duration_ms: started_at.elapsed().as_millis() as u64,
                truncated: false,
                error: Some(err.clone()),
                leak_guard: None,
                created_at: AgentRuntime::now_epoch_seconds(),
            });
            bail!(err);
        }
    }

    Ok(())
}

pub(super) fn emit_tool_approval_warning(
    runtime: &AgentRuntime,
    session_id: &str,
    tool_name: &str,
    args_json: &str,
) -> anyhow::Result<()> {
    if let Some(warning) =
        runtime.evaluate_tool_approval(session_id, tool_name, args_json, false)?
    {
        let _ = runtime.append_acp_event(AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "approval.warn".to_string(),
            payload: serde_json::json!({
                "tool": tool_name,
                "message": warning,
            }),
            created_at: AgentRuntime::now_epoch_seconds(),
        });
    }
    Ok(())
}

pub(super) fn emit_tool_started(runtime: &AgentRuntime, session_id: &str, tool_name: &str) {
    let _ = runtime.append_acp_event(AcpEventRecord {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: Some(session_id.to_string()),
        event_type: "tool.started".to_string(),
        payload: serde_json::json!({
            "tool": tool_name,
        }),
        created_at: AgentRuntime::now_epoch_seconds(),
    });
}

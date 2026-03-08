use rexos_kernel::router::TaskKind;

use crate::records::AcpEventRecord;
use crate::AgentRuntime;

pub(super) fn session_kind_label(kind: TaskKind) -> String {
    format!("{kind:?}").to_lowercase()
}

impl AgentRuntime {
    pub(crate) fn append_session_started_event(
        &self,
        session_id: &str,
        kind: TaskKind,
        user_prompt: &str,
    ) {
        let _ = self.append_acp_event(AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "session.started".to_string(),
            payload: serde_json::json!({
                "kind": session_kind_label(kind),
                "user_prompt_chars": user_prompt.chars().count(),
            }),
            created_at: Self::now_epoch_seconds(),
        });
    }

    pub(crate) fn append_session_completed_event(&self, session_id: &str, output: &str) {
        let _ = self.append_acp_event(AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "session.completed".to_string(),
            payload: serde_json::json!({
                "output_chars": output.chars().count(),
                "reason": "assistant_stop",
            }),
            created_at: Self::now_epoch_seconds(),
        });
    }

    pub(crate) fn append_session_failed_event(&self, session_id: &str) {
        let _ = self.append_acp_event(AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "session.failed".to_string(),
            payload: serde_json::json!({
                "reason": "max_iterations_exceeded",
            }),
            created_at: Self::now_epoch_seconds(),
        });
    }
}

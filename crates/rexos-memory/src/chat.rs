use rexos_llm::openai_compat::{ChatMessage, Role, ToolCall};

use crate::MemoryStore;

use super::time::now_epoch_seconds;

impl MemoryStore {
    pub fn append_chat_message(&self, session_id: &str, msg: &ChatMessage) -> anyhow::Result<()> {
        let now = now_epoch_seconds().to_string();

        self.conn.execute(
            "INSERT INTO sessions (session_id, created_at) VALUES (?1, ?2)\n            ON CONFLICT(session_id) DO NOTHING",
            (session_id, &now),
        )?;

        let role = role_to_str(msg.role);
        let content = msg.content.clone().unwrap_or_default();
        let tool_calls_json = msg
            .tool_calls
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;

        self.conn.execute(
            "INSERT INTO messages (session_id, role, content, created_at, name, tool_call_id, tool_calls_json)\n            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                session_id,
                role,
                content,
                &now,
                msg.name.as_deref(),
                msg.tool_call_id.as_deref(),
                tool_calls_json.as_deref(),
            ),
        )?;

        Ok(())
    }

    pub fn list_chat_messages(&self, session_id: &str) -> anyhow::Result<Vec<ChatMessage>> {
        let msgs = self.list_messages(session_id)?;
        let mut out = Vec::with_capacity(msgs.len());

        for m in msgs {
            let role = role_from_str(&m.role)?;
            let tool_calls = match m.tool_calls_json.as_deref() {
                Some(s) if !s.trim().is_empty() => Some(serde_json::from_str::<Vec<ToolCall>>(s)?),
                _ => None,
            };

            let content = if m.content.is_empty() && tool_calls.is_some() {
                None
            } else {
                Some(m.content)
            };

            out.push(ChatMessage {
                role,
                content,
                name: m.name,
                tool_call_id: m.tool_call_id,
                tool_calls,
            });
        }

        Ok(out)
    }
}

fn role_to_str(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

fn role_from_str(s: &str) -> anyhow::Result<Role> {
    match s {
        "system" => Ok(Role::System),
        "user" => Ok(Role::User),
        "assistant" => Ok(Role::Assistant),
        "tool" => Ok(Role::Tool),
        _ => anyhow::bail!("unknown role: {s}"),
    }
}

use crate::{MemoryStore, Message};

use super::time::now_epoch_seconds;

impl MemoryStore {
    pub fn append_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> anyhow::Result<()> {
        let now = now_epoch_seconds().to_string();

        self.conn.execute(
            "INSERT INTO sessions (session_id, created_at) VALUES (?1, ?2)\n            ON CONFLICT(session_id) DO NOTHING",
            (session_id, &now),
        )?;

        self.conn.execute(
            "INSERT INTO messages (session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4)",
            (session_id, role, content, &now),
        )?;

        Ok(())
    }

    pub fn list_messages(&self, session_id: &str) -> anyhow::Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, role, content, created_at, name, tool_call_id, tool_calls_json FROM messages WHERE session_id=?1 ORDER BY id ASC",
        )?;

        let mut rows = stmt.query((session_id,))?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(Message {
                id: row.get(0)?,
                session_id: session_id.to_string(),
                role: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                name: row.get(4)?,
                tool_call_id: row.get(5)?,
                tool_calls_json: row.get(6)?,
            });
        }
        Ok(out)
    }
}

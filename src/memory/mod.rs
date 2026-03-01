use std::path::Path;

use anyhow::Context;
use rusqlite::{Connection, OptionalExtension};

use crate::paths::RexosPaths;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: i64,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn open_or_create(paths: &RexosPaths) -> anyhow::Result<Self> {
        Self::open_or_create_at_path(&paths.db_path())
    }

    fn open_or_create_at_path(path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("open sqlite db: {}", path.display()))?;

        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> anyhow::Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA journal_mode=WAL;

            CREATE TABLE IF NOT EXISTS kv (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS sessions (
              session_id TEXT PRIMARY KEY,
              created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              session_id TEXT NOT NULL,
              role TEXT NOT NULL,
              content TEXT NOT NULL,
              created_at TEXT NOT NULL,
              FOREIGN KEY (session_id) REFERENCES sessions(session_id)
            );
            CREATE INDEX IF NOT EXISTS idx_messages_session_id ON messages(session_id);
            "#,
        )?;
        Ok(())
    }

    pub fn kv_set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO kv (key, value) VALUES (?1, ?2)\n            ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            (key, value),
        )?;
        Ok(())
    }

    pub fn kv_get(&self, key: &str) -> anyhow::Result<Option<String>> {
        let value = self
            .conn
            .query_row("SELECT value FROM kv WHERE key=?1", (key,), |row| row.get(0))
            .optional()?;
        Ok(value)
    }

    pub fn append_message(&self, session_id: &str, role: &str, content: &str) -> anyhow::Result<()> {
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
            "SELECT id, role, content, created_at FROM messages WHERE session_id=?1 ORDER BY id ASC",
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
            });
        }
        Ok(out)
    }
}

fn now_epoch_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn kv_round_trip() {
        let tmp = tempdir().unwrap();
        let db_path = tmp.path().join("test.db");
        let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();

        assert_eq!(store.kv_get("missing").unwrap(), None);
        store.kv_set("a", "1").unwrap();
        assert_eq!(store.kv_get("a").unwrap(), Some("1".to_string()));
        store.kv_set("a", "2").unwrap();
        assert_eq!(store.kv_get("a").unwrap(), Some("2".to_string()));
    }

    #[test]
    fn messages_persist_across_reopen() {
        let tmp = tempdir().unwrap();
        let db_path = tmp.path().join("test.db");

        {
            let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();
            store.append_message("s1", "user", "hello").unwrap();
            store.append_message("s1", "assistant", "world").unwrap();
        }

        let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();
        let msgs = store.list_messages("s1").unwrap();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, "user");
        assert_eq!(msgs[0].content, "hello");
        assert_eq!(msgs[1].role, "assistant");
        assert_eq!(msgs[1].content, "world");
    }
}

use rusqlite::Connection;

pub(super) fn migrate(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
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
          name TEXT,
          tool_call_id TEXT,
          tool_calls_json TEXT,
          FOREIGN KEY (session_id) REFERENCES sessions(session_id)
        );
        CREATE INDEX IF NOT EXISTS idx_messages_session_id ON messages(session_id);
        "#,
    )?;

    let _ = conn.execute("ALTER TABLE messages ADD COLUMN name TEXT", ());
    let _ = conn.execute("ALTER TABLE messages ADD COLUMN tool_call_id TEXT", ());
    let _ = conn.execute("ALTER TABLE messages ADD COLUMN tool_calls_json TEXT", ());

    Ok(())
}

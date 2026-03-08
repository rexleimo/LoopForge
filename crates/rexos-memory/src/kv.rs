use rusqlite::OptionalExtension;

use crate::MemoryStore;

impl MemoryStore {
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
            .query_row("SELECT value FROM kv WHERE key=?1", (key,), |row| {
                row.get(0)
            })
            .optional()?;
        Ok(value)
    }
}

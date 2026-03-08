use std::path::Path;

use anyhow::Context;
use rexos_kernel::paths::RexosPaths;
use rusqlite::Connection;

use crate::MemoryStore;

use super::schema;

impl MemoryStore {
    pub fn open_or_create(paths: &RexosPaths) -> anyhow::Result<Self> {
        Self::open_or_create_at_path(&paths.db_path())
    }

    pub(super) fn open_or_create_at_path(path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("open sqlite db: {}", path.display()))?;

        schema::migrate(&conn)?;
        Ok(Self { conn })
    }
}

mod chat;
mod kv;
mod messages;
mod schema;
mod store;
mod time;

#[cfg(test)]
mod tests;

use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: i64,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls_json: Option<String>,
}

#[derive(Debug)]
pub struct MemoryStore {
    conn: Connection,
}

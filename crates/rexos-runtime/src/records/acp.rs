#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcpEventRecord {
    pub id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcpDeliveryCheckpointRecord {
    pub channel: String,
    pub cursor: String,
    pub updated_at: i64,
}

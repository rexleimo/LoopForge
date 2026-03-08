#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OutboxStatus {
    Queued,
    Sent,
    Failed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct OutboxMessageRecord {
    pub(crate) message_id: String,
    #[serde(default)]
    pub(crate) session_id: Option<String>,
    pub(crate) channel: String,
    pub(crate) recipient: String,
    #[serde(default)]
    pub(crate) subject: Option<String>,
    pub(crate) message: String,
    pub(crate) status: OutboxStatus,
    pub(crate) attempts: u32,
    #[serde(default)]
    pub(crate) last_error: Option<String>,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
    #[serde(default)]
    pub(crate) sent_at: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ChannelSendToolArgs {
    pub(crate) channel: String,
    pub(crate) recipient: String,
    #[serde(default)]
    pub(crate) subject: Option<String>,
    pub(crate) message: String,
}

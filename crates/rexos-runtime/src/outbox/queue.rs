use super::store::{self, outbox_messages_get, outbox_messages_set};
use crate::records::{ChannelSendToolArgs, OutboxMessageRecord, OutboxStatus};
use crate::AgentRuntime;

fn validate_channel_send(args: &ChannelSendToolArgs) -> Option<String> {
    if args.channel.trim().is_empty() {
        return Some("error: channel is empty".to_string());
    }
    if args.recipient.trim().is_empty() {
        return Some("error: recipient is empty".to_string());
    }
    if args.message.trim().is_empty() {
        return Some("error: message is empty".to_string());
    }

    match args.channel.as_str() {
        "console" | "webhook" => None,
        other => Some(format!("error: unknown channel: {other}")),
    }
}

fn build_outbox_record(session_id: Option<&str>, args: ChannelSendToolArgs) -> OutboxMessageRecord {
    let now = AgentRuntime::now_epoch_seconds();
    OutboxMessageRecord {
        message_id: uuid::Uuid::new_v4().to_string(),
        session_id: session_id.map(|value| value.to_string()),
        channel: args.channel,
        recipient: args.recipient,
        subject: args.subject.filter(|value| !value.trim().is_empty()),
        message: args.message,
        status: OutboxStatus::Queued,
        attempts: 0,
        last_error: None,
        created_at: now,
        updated_at: now,
        sent_at: None,
    }
}

impl AgentRuntime {
    pub(crate) fn channel_send(
        &self,
        session_id: Option<&str>,
        args: ChannelSendToolArgs,
    ) -> anyhow::Result<String> {
        if let Some(error) = validate_channel_send(&args) {
            return Ok(error);
        }

        let record = build_outbox_record(session_id, args);
        let mut msgs = outbox_messages_get(&self.memory)?;
        msgs.push(record.clone());
        if msgs.len() > store::OUTBOX_MAX_MESSAGES {
            msgs.drain(0..(msgs.len() - store::OUTBOX_MAX_MESSAGES));
        }
        outbox_messages_set(&self.memory, &msgs)?;

        Ok(serde_json::json!({
            "status": "queued",
            "message_id": record.message_id,
        })
        .to_string())
    }
}

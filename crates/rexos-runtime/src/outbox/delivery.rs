use anyhow::Context;
use rexos_memory::MemoryStore;

use crate::acp::{acp_delivery_checkpoints_get, acp_delivery_checkpoints_set};
use crate::records::{AcpDeliveryCheckpointRecord, OutboxMessageRecord};
use crate::AgentRuntime;

pub(super) fn upsert_acp_delivery_checkpoint(
    memory: &MemoryStore,
    session_id: &str,
    channel: &str,
    cursor: &str,
) -> anyhow::Result<()> {
    let mut checkpoints = acp_delivery_checkpoints_get(memory, session_id)?;
    let now = AgentRuntime::now_epoch_seconds();
    if let Some(existing) = checkpoints
        .iter_mut()
        .find(|checkpoint| checkpoint.channel == channel)
    {
        existing.cursor = cursor.to_string();
        existing.updated_at = now;
    } else {
        checkpoints.push(AcpDeliveryCheckpointRecord {
            channel: channel.to_string(),
            cursor: cursor.to_string(),
            updated_at: now,
        });
    }
    acp_delivery_checkpoints_set(memory, session_id, &checkpoints)
}

pub(super) fn deliver_console(msg: &OutboxMessageRecord) {
    let subject = msg.subject.as_deref().unwrap_or("");
    println!(
        "[rexos][channel_send][console] to={} subject={} message={}",
        msg.recipient, subject, msg.message
    );
}

pub(super) async fn deliver_webhook(
    http: &reqwest::Client,
    msg: &OutboxMessageRecord,
) -> anyhow::Result<()> {
    let mut payload = serde_json::json!({
        "message": msg.message,
        "recipient": msg.recipient,
        "message_id": msg.message_id,
    });
    if let Some(subject) = &msg.subject {
        payload["subject"] = serde_json::Value::String(subject.clone());
    }
    http.post(&msg.recipient)
        .json(&payload)
        .send()
        .await
        .context("send webhook request")?
        .error_for_status()
        .context("webhook response status")?;
    Ok(())
}

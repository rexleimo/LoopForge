use crate::acp::append_acp_event;
use crate::outbox::delivery::upsert_acp_delivery_checkpoint;
use crate::records::{AcpEventRecord, OutboxMessageRecord};
use rexos_memory::MemoryStore;

pub(super) fn record_delivery_sent(memory: &MemoryStore, msg: &OutboxMessageRecord, now: i64) {
    let Some(session_id) = msg.session_id.as_deref() else {
        return;
    };

    let _ = upsert_acp_delivery_checkpoint(memory, session_id, &msg.channel, &msg.message_id);
    let _ = append_acp_event(
        memory,
        AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "delivery.sent".to_string(),
            payload: serde_json::json!({
                "channel": msg.channel.clone(),
                "message_id": msg.message_id.clone(),
                "recipient": msg.recipient.clone(),
            }),
            created_at: now,
        },
    );
}

pub(super) fn record_delivery_failed(memory: &MemoryStore, msg: &OutboxMessageRecord, now: i64) {
    let Some(session_id) = msg.session_id.as_deref() else {
        return;
    };

    let _ = append_acp_event(
        memory,
        AcpEventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: Some(session_id.to_string()),
            event_type: "delivery.failed".to_string(),
            payload: serde_json::json!({
                "channel": msg.channel.clone(),
                "message_id": msg.message_id.clone(),
                "recipient": msg.recipient.clone(),
                "error": msg.last_error.clone(),
            }),
            created_at: now,
        },
    );
}

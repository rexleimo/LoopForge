use anyhow::Context;
use rexos_memory::MemoryStore;

use crate::records::OutboxMessageRecord;

const OUTBOX_MESSAGES_KEY: &str = "rexos.outbox.messages";
pub(super) const OUTBOX_MAX_MESSAGES: usize = 500;

pub(super) fn outbox_messages_get(
    memory: &MemoryStore,
) -> anyhow::Result<Vec<OutboxMessageRecord>> {
    let raw = memory
        .kv_get(OUTBOX_MESSAGES_KEY)
        .context("kv_get rexos.outbox.messages")?
        .unwrap_or_else(|| "[]".to_string());
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

pub(super) fn outbox_messages_set(
    memory: &MemoryStore,
    msgs: &[OutboxMessageRecord],
) -> anyhow::Result<()> {
    let raw = serde_json::to_string(msgs).context("serialize rexos.outbox.messages")?;
    memory
        .kv_set(OUTBOX_MESSAGES_KEY, &raw)
        .context("kv_set rexos.outbox.messages")?;
    Ok(())
}

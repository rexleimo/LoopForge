use anyhow::Context;
use rexos_memory::MemoryStore;

use crate::records::AcpEventRecord;
use crate::{ACP_EVENTS_KEY, ACP_EVENTS_MAX_RECORDS};

pub(super) fn acp_events_get(memory: &MemoryStore) -> anyhow::Result<Vec<AcpEventRecord>> {
    let raw = memory
        .kv_get(ACP_EVENTS_KEY)
        .context("kv_get acp events")?
        .unwrap_or_else(|| "[]".to_string());
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

pub(super) fn acp_events_set(
    memory: &MemoryStore,
    events: &[AcpEventRecord],
) -> anyhow::Result<()> {
    let raw = serde_json::to_string(events).context("serialize acp events")?;
    memory
        .kv_set(ACP_EVENTS_KEY, &raw)
        .context("kv_set acp events")?;
    Ok(())
}

pub(super) fn append_acp_event(memory: &MemoryStore, record: AcpEventRecord) -> anyhow::Result<()> {
    let mut events = acp_events_get(memory)?;
    events.push(record);
    if events.len() > ACP_EVENTS_MAX_RECORDS {
        events.drain(0..(events.len() - ACP_EVENTS_MAX_RECORDS));
    }
    acp_events_set(memory, &events)
}

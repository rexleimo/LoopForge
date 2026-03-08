mod checkpoints;
mod events;

#[cfg(test)]
mod tests;

use rexos_memory::MemoryStore;

use crate::records::{AcpDeliveryCheckpointRecord, AcpEventRecord};

pub(crate) fn acp_events_get(memory: &MemoryStore) -> anyhow::Result<Vec<AcpEventRecord>> {
    events::acp_events_get(memory)
}

pub(crate) fn append_acp_event(memory: &MemoryStore, record: AcpEventRecord) -> anyhow::Result<()> {
    events::append_acp_event(memory, record)
}

pub(crate) fn acp_delivery_checkpoints_get(
    memory: &MemoryStore,
    session_id: &str,
) -> anyhow::Result<Vec<AcpDeliveryCheckpointRecord>> {
    checkpoints::acp_delivery_checkpoints_get(memory, session_id)
}

pub(crate) fn acp_delivery_checkpoints_set(
    memory: &MemoryStore,
    session_id: &str,
    checkpoints: &[AcpDeliveryCheckpointRecord],
) -> anyhow::Result<()> {
    checkpoints::acp_delivery_checkpoints_set(memory, session_id, checkpoints)
}

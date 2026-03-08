use anyhow::Context;
use rexos_memory::MemoryStore;

use crate::records::AcpDeliveryCheckpointRecord;
use crate::ACP_CHECKPOINTS_KEY_PREFIX;

fn acp_checkpoints_key(session_id: &str) -> String {
    format!("{ACP_CHECKPOINTS_KEY_PREFIX}{session_id}")
}

pub(super) fn acp_delivery_checkpoints_get(
    memory: &MemoryStore,
    session_id: &str,
) -> anyhow::Result<Vec<AcpDeliveryCheckpointRecord>> {
    let raw = memory
        .kv_get(&acp_checkpoints_key(session_id))
        .context("kv_get acp delivery checkpoints")?
        .unwrap_or_else(|| "[]".to_string());
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

pub(super) fn acp_delivery_checkpoints_set(
    memory: &MemoryStore,
    session_id: &str,
    checkpoints: &[AcpDeliveryCheckpointRecord],
) -> anyhow::Result<()> {
    let raw = serde_json::to_string(checkpoints).context("serialize acp delivery checkpoints")?;
    memory
        .kv_set(&acp_checkpoints_key(session_id), &raw)
        .context("kv_set acp delivery checkpoints")?;
    Ok(())
}

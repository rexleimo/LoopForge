use anyhow::Context;
use rexos_memory::MemoryStore;

use crate::acp::{acp_delivery_checkpoints_get, acp_events_get, append_acp_event};
use crate::records::{SkillAuditRecord, ToolAuditRecord};
use crate::{
    AcpDeliveryCheckpointRecord, AcpEventRecord, AgentRuntime, SKILL_AUDIT_KEY,
    SKILL_AUDIT_MAX_RECORDS, TOOL_AUDIT_KEY, TOOL_AUDIT_MAX_RECORDS,
};

pub(crate) fn append_tool_audit(
    memory: &MemoryStore,
    record: ToolAuditRecord,
) -> anyhow::Result<()> {
    let raw = memory
        .kv_get(TOOL_AUDIT_KEY)
        .context("kv_get tool audit")?
        .unwrap_or_else(|| "[]".to_string());
    let mut records: Vec<ToolAuditRecord> = serde_json::from_str(&raw).unwrap_or_default();
    records.push(record);
    if records.len() > TOOL_AUDIT_MAX_RECORDS {
        records.drain(0..(records.len() - TOOL_AUDIT_MAX_RECORDS));
    }
    let serialized = serde_json::to_string(&records).context("serialize tool audit")?;
    memory
        .kv_set(TOOL_AUDIT_KEY, &serialized)
        .context("kv_set tool audit")?;
    Ok(())
}

pub(crate) fn append_skill_audit(
    memory: &MemoryStore,
    record: SkillAuditRecord,
) -> anyhow::Result<()> {
    let raw = memory
        .kv_get(SKILL_AUDIT_KEY)
        .context("kv_get skill audit")?
        .unwrap_or_else(|| "[]".to_string());
    let mut records: Vec<SkillAuditRecord> = serde_json::from_str(&raw).unwrap_or_default();
    records.push(record);
    if records.len() > SKILL_AUDIT_MAX_RECORDS {
        records.drain(0..(records.len() - SKILL_AUDIT_MAX_RECORDS));
    }
    let serialized = serde_json::to_string(&records).context("serialize skill audit")?;
    memory
        .kv_set(SKILL_AUDIT_KEY, &serialized)
        .context("kv_set skill audit")?;
    Ok(())
}

pub(crate) fn list_acp_events(
    memory: &MemoryStore,
    session_id: Option<&str>,
    limit: usize,
) -> anyhow::Result<Vec<AcpEventRecord>> {
    let mut events = acp_events_get(memory)?;
    if let Some(session_id) = session_id {
        let session_id = session_id.trim();
        if !session_id.is_empty() {
            events.retain(|event| event.session_id.as_deref() == Some(session_id));
        }
    }
    let wanted = limit.max(1);
    if events.len() > wanted {
        events = events.split_off(events.len() - wanted);
    }
    Ok(events)
}

pub(crate) fn list_acp_delivery_checkpoints(
    memory: &MemoryStore,
    session_id: &str,
) -> anyhow::Result<Vec<AcpDeliveryCheckpointRecord>> {
    acp_delivery_checkpoints_get(memory, session_id)
}

impl AgentRuntime {
    pub(crate) fn append_tool_audit(&self, record: ToolAuditRecord) -> anyhow::Result<()> {
        append_tool_audit(&self.memory, record)
    }

    pub(crate) fn append_skill_audit(&self, record: SkillAuditRecord) -> anyhow::Result<()> {
        append_skill_audit(&self.memory, record)
    }

    pub(crate) fn append_acp_event(&self, record: AcpEventRecord) -> anyhow::Result<()> {
        append_acp_event(&self.memory, record)
    }

    pub fn list_acp_events(
        &self,
        session_id: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<AcpEventRecord>> {
        list_acp_events(&self.memory, session_id, limit)
    }

    pub fn list_acp_delivery_checkpoints(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Vec<AcpDeliveryCheckpointRecord>> {
        list_acp_delivery_checkpoints(&self.memory, session_id)
    }
}

use anyhow::Context;

use crate::records::HandInstanceRecord;
use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) fn hands_instances_index(&self) -> anyhow::Result<Vec<String>> {
        let raw = self
            .memory
            .kv_get("rexos.hands.instances.index")
            .context("kv_get rexos.hands.instances.index")?
            .unwrap_or_else(|| "[]".to_string());
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    }

    pub(super) fn put_hands_instances_index(&self, ids: &[String]) -> anyhow::Result<()> {
        let raw = serde_json::to_string(ids).context("serialize hands instances index")?;
        self.memory
            .kv_set("rexos.hands.instances.index", &raw)
            .context("kv_set rexos.hands.instances.index")?;
        Ok(())
    }

    pub(super) fn hand_instance_key(instance_id: &str) -> String {
        format!("rexos.hands.instances.{instance_id}")
    }

    pub(super) fn get_hand_instance(
        &self,
        instance_id: &str,
    ) -> anyhow::Result<Option<HandInstanceRecord>> {
        let raw = self
            .memory
            .kv_get(&Self::hand_instance_key(instance_id))
            .with_context(|| format!("kv_get hand instance {instance_id}"))?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let record: HandInstanceRecord = serde_json::from_str(&raw)
            .with_context(|| format!("parse hand instance {instance_id}"))?;
        Ok(Some(record))
    }

    pub(super) fn put_hand_instance(&self, record: &HandInstanceRecord) -> anyhow::Result<()> {
        let raw = serde_json::to_string(record).context("serialize hand instance record")?;
        self.memory
            .kv_set(&Self::hand_instance_key(&record.instance_id), &raw)
            .with_context(|| format!("kv_set hand instance {}", record.instance_id))?;
        Ok(())
    }
}

use anyhow::Context;

use crate::records::AgentRecord;
use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) fn agents_index(&self) -> anyhow::Result<Vec<String>> {
        let raw = self
            .memory
            .kv_get("rexos.agents.index")
            .context("kv_get rexos.agents.index")?
            .unwrap_or_else(|| "[]".to_string());
        let ids: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
        Ok(ids)
    }

    pub(super) fn put_agents_index(&self, ids: &[String]) -> anyhow::Result<()> {
        let raw = serde_json::to_string(ids).context("serialize agents index")?;
        self.memory
            .kv_set("rexos.agents.index", &raw)
            .context("kv_set rexos.agents.index")?;
        Ok(())
    }

    pub(super) fn agent_key(agent_id: &str) -> String {
        format!("rexos.agents.{agent_id}")
    }

    pub(super) fn get_agent(&self, agent_id: &str) -> anyhow::Result<Option<AgentRecord>> {
        let raw = self
            .memory
            .kv_get(&Self::agent_key(agent_id))
            .with_context(|| format!("kv_get agent {agent_id}"))?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let record: AgentRecord =
            serde_json::from_str(&raw).with_context(|| format!("parse agent {agent_id}"))?;
        Ok(Some(record))
    }

    pub(super) fn put_agent(&self, record: &AgentRecord) -> anyhow::Result<()> {
        let raw = serde_json::to_string(record).context("serialize agent record")?;
        self.memory
            .kv_set(&Self::agent_key(&record.id), &raw)
            .with_context(|| format!("kv_set agent {}", record.id))?;
        Ok(())
    }
}

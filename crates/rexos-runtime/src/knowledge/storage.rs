use anyhow::Context;

use crate::records::{KnowledgeEntityRecord, KnowledgeRelationRecord};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) fn knowledge_entities_get(&self) -> anyhow::Result<Vec<KnowledgeEntityRecord>> {
        let key = "rexos.knowledge.entities";
        let raw = self
            .memory
            .kv_get(key)
            .context("kv_get rexos.knowledge.entities")?
            .unwrap_or_else(|| "[]".to_string());
        let entities: Vec<KnowledgeEntityRecord> = serde_json::from_str(&raw).unwrap_or_default();
        Ok(entities)
    }

    pub(super) fn knowledge_entities_set(
        &self,
        entities: &[KnowledgeEntityRecord],
    ) -> anyhow::Result<()> {
        let key = "rexos.knowledge.entities";
        let raw = serde_json::to_string(entities).context("serialize rexos.knowledge.entities")?;
        self.memory
            .kv_set(key, &raw)
            .context("kv_set rexos.knowledge.entities")?;
        Ok(())
    }

    pub(super) fn knowledge_relations_get(&self) -> anyhow::Result<Vec<KnowledgeRelationRecord>> {
        let key = "rexos.knowledge.relations";
        let raw = self
            .memory
            .kv_get(key)
            .context("kv_get rexos.knowledge.relations")?
            .unwrap_or_else(|| "[]".to_string());
        let relations: Vec<KnowledgeRelationRecord> =
            serde_json::from_str(&raw).unwrap_or_default();
        Ok(relations)
    }

    pub(super) fn knowledge_relations_set(
        &self,
        relations: &[KnowledgeRelationRecord],
    ) -> anyhow::Result<()> {
        let key = "rexos.knowledge.relations";
        let raw =
            serde_json::to_string(relations).context("serialize rexos.knowledge.relations")?;
        self.memory
            .kv_set(key, &raw)
            .context("kv_set rexos.knowledge.relations")?;
        Ok(())
    }
}

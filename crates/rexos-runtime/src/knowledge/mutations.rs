use crate::records::{
    KnowledgeAddEntityToolArgs, KnowledgeAddRelationToolArgs, KnowledgeEntityRecord,
    KnowledgeRelationRecord,
};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(crate) fn knowledge_add_entity(
        &self,
        args: KnowledgeAddEntityToolArgs,
    ) -> anyhow::Result<String> {
        let id = args.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let mut entities = self.knowledge_entities_get()?;
        if let Some(existing) = entities.iter().find(|entity| entity.id == id) {
            return Ok(serde_json::to_string(existing).unwrap_or_else(|_| "ok".to_string()));
        }

        let now = Self::now_epoch_seconds();
        let record = KnowledgeEntityRecord {
            id: id.clone(),
            name: args.name,
            entity_type: args.entity_type,
            properties: args.properties,
            created_at: now,
            updated_at: now,
        };

        entities.push(record.clone());
        self.knowledge_entities_set(&entities)?;

        Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()))
    }

    pub(crate) fn knowledge_add_relation(
        &self,
        args: KnowledgeAddRelationToolArgs,
    ) -> anyhow::Result<String> {
        let id = args.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let mut relations = self.knowledge_relations_get()?;
        if let Some(existing) = relations.iter().find(|relation| relation.id == id) {
            return Ok(serde_json::to_string(existing).unwrap_or_else(|_| "ok".to_string()));
        }

        let record = KnowledgeRelationRecord {
            id: id.clone(),
            source: args.source,
            relation: args.relation,
            target: args.target,
            properties: args.properties,
            created_at: Self::now_epoch_seconds(),
        };

        relations.push(record.clone());
        self.knowledge_relations_set(&relations)?;

        Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()))
    }
}

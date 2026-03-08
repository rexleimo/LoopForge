use std::collections::HashSet;

use crate::records::{KnowledgeEntityRecord, KnowledgeRelationRecord};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(crate) fn knowledge_query(&self, query: &str) -> anyhow::Result<String> {
        let normalized_query = query.trim().to_lowercase();
        if normalized_query.is_empty() {
            return Ok(r#"{"entities":[],"relations":[]}"#.to_string());
        }

        let entities = self.knowledge_entities_get()?;
        let relations = self.knowledge_relations_get()?;

        let matched_entities: Vec<KnowledgeEntityRecord> = entities
            .into_iter()
            .filter(|entity| {
                entity.id.to_lowercase().contains(&normalized_query)
                    || entity.name.to_lowercase().contains(&normalized_query)
                    || entity
                        .entity_type
                        .to_lowercase()
                        .contains(&normalized_query)
            })
            .collect();

        let matched_entity_ids: HashSet<String> = matched_entities
            .iter()
            .map(|entity| entity.id.clone())
            .collect();

        let matched_relations: Vec<KnowledgeRelationRecord> = relations
            .into_iter()
            .filter(|relation| {
                relation.id.to_lowercase().contains(&normalized_query)
                    || relation.source.to_lowercase().contains(&normalized_query)
                    || relation.target.to_lowercase().contains(&normalized_query)
                    || relation.relation.to_lowercase().contains(&normalized_query)
                    || matched_entity_ids.contains(&relation.source)
                    || matched_entity_ids.contains(&relation.target)
            })
            .collect();

        Ok(serde_json::json!({
            "entities": matched_entities,
            "relations": matched_relations,
        })
        .to_string())
    }
}

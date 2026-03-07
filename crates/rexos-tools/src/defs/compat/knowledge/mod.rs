mod entity;
mod query;
mod relation;
mod shared;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![
        entity::knowledge_add_entity_def(),
        relation::knowledge_add_relation_def(),
        query::knowledge_query_def(),
    ]
}

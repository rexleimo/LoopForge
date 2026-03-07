use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn knowledge_add_entity_def() -> ToolDefinition {
    compat_function_def(
        "knowledge_add_entity",
        "Add an entity to the shared knowledge graph.",
        json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Optional stable entity id. If omitted, LoopForge generates one." },
                "name": { "type": "string", "description": "Entity name." },
                "entity_type": { "type": "string", "description": "Entity type (free-form string)." },
                "properties": { "type": "object", "description": "Optional properties map.", "additionalProperties": true }
            },
            "required": ["name", "entity_type"],
            "additionalProperties": false
        }),
    )
}

use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn knowledge_add_relation_def() -> ToolDefinition {
    compat_function_def(
        "knowledge_add_relation",
        "Add a relation to the shared knowledge graph.",
        json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Optional stable relation id. If omitted, LoopForge generates one." },
                "source": { "type": "string", "description": "Source entity id." },
                "relation": { "type": "string", "description": "Relation type/name." },
                "target": { "type": "string", "description": "Target entity id." },
                "properties": { "type": "object", "description": "Optional properties map.", "additionalProperties": true }
            },
            "required": ["source", "relation", "target"],
            "additionalProperties": false
        }),
    )
}

use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn knowledge_query_def() -> ToolDefinition {
    compat_function_def(
        "knowledge_query",
        "Query the shared knowledge graph.",
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Query string (substring match)." }
            },
            "required": ["query"],
            "additionalProperties": false
        }),
    )
}

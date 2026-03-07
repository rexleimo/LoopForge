use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn agent_find_def() -> ToolDefinition {
    compat_function_def(
        "agent_find",
        "Find agent sessions by id or name (substring match).",
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search query (case-insensitive substring)." }
            },
            "required": ["query"],
            "additionalProperties": false
        }),
    )
}

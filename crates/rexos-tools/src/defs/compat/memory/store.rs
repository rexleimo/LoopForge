use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

pub(super) fn tool_def() -> ToolDefinition {
    super::function_def(
        "memory_store",
        "Persist a key/value pair to shared memory.",
        json!({
            "type": "object",
            "properties": {
                "key": { "type": "string", "description": "The memory key" },
                "value": { "type": "string", "description": "The value to store" }
            },
            "required": ["key", "value"],
            "additionalProperties": false
        }),
    )
}

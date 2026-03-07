use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

pub(super) fn tool_def() -> ToolDefinition {
    super::function_def(
        "memory_recall",
        "Recall a value from shared memory.",
        json!({
            "type": "object",
            "properties": {
                "key": { "type": "string", "description": "The memory key" }
            },
            "required": ["key"],
            "additionalProperties": false
        }),
    )
}

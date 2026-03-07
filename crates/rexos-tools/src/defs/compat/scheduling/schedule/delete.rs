use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "schedule_delete",
        "Delete a schedule entry by id.",
        json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Schedule id." }
            },
            "required": ["id"],
            "additionalProperties": false
        }),
    )
}

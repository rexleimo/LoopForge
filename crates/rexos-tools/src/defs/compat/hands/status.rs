use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn hand_status_def() -> ToolDefinition {
    compat_function_def(
        "hand_status",
        "Get status for a Hand by id.",
        json!({
            "type": "object",
            "properties": {
                "hand_id": { "type": "string", "description": "Hand id." }
            },
            "required": ["hand_id"],
            "additionalProperties": false
        }),
    )
}

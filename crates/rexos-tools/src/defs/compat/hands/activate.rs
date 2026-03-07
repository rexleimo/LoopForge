use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn hand_activate_def() -> ToolDefinition {
    compat_function_def(
        "hand_activate",
        "Activate a Hand (spawns a specialized agent instance).",
        json!({
            "type": "object",
            "properties": {
                "hand_id": { "type": "string", "description": "Hand id (e.g. 'browser', 'coder')." },
                "config": { "type": "object", "description": "Optional hand configuration (stored and appended to the hand system prompt)." }
            },
            "required": ["hand_id"],
            "additionalProperties": false
        }),
    )
}

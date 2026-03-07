use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn hand_deactivate_def() -> ToolDefinition {
    compat_function_def(
        "hand_deactivate",
        "Deactivate a running Hand instance.",
        json!({
            "type": "object",
            "properties": {
                "instance_id": { "type": "string", "description": "Hand instance id returned by hand_activate." }
            },
            "required": ["instance_id"],
            "additionalProperties": false
        }),
    )
}

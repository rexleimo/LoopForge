use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn agent_kill_def() -> ToolDefinition {
    compat_function_def(
        "agent_kill",
        "Mark an agent session as killed.",
        json!({
            "type": "object",
            "properties": {
                "agent_id": { "type": "string", "description": "Target agent id." }
            },
            "required": ["agent_id"],
            "additionalProperties": false
        }),
    )
}

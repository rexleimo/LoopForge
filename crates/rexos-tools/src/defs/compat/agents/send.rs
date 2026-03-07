use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn agent_send_def() -> ToolDefinition {
    compat_function_def(
        "agent_send",
        "Send a message to an agent session and return its response.",
        json!({
            "type": "object",
            "properties": {
                "agent_id": { "type": "string", "description": "Target agent id." },
                "message": { "type": "string", "description": "Message to send." }
            },
            "required": ["agent_id", "message"],
            "additionalProperties": false
        }),
    )
}

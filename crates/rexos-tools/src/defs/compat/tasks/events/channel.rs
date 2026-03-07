use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "channel_send",
        "Enqueue an outbound message into the outbox (delivery happens via dispatcher).",
        json!({
            "type": "object",
            "properties": {
                "channel": { "type": "string", "description": "Channel adapter name (console, webhook)." },
                "recipient": { "type": "string", "description": "Channel-specific recipient identifier." },
                "subject": { "type": "string", "description": "Optional subject line (used by some channels)." },
                "message": { "type": "string", "description": "Message body to send." }
            },
            "required": ["channel", "recipient", "message"],
            "additionalProperties": false
        }),
    )
}

use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "event_publish",
        "Publish an event into the shared event log.",
        json!({
            "type": "object",
            "properties": {
                "event_type": { "type": "string", "description": "Event type/name." },
                "payload": { "type": "object", "description": "Optional event payload." }
            },
            "required": ["event_type"],
            "additionalProperties": false
        }),
    )
}

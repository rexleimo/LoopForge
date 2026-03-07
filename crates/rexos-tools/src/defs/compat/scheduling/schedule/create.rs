use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "schedule_create",
        "Create a schedule entry (persisted).",
        json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Optional stable schedule id. If omitted, LoopForge generates one." },
                "description": { "type": "string", "description": "Human-readable description." },
                "schedule": { "type": "string", "description": "Schedule expression (stored as-is)." },
                "agent_id": { "type": "string", "description": "Optional agent id to associate with this schedule." },
                "agent": { "type": "string", "description": "Alias of agent_id (optional)." },
                "enabled": { "type": "boolean", "description": "Whether this schedule is enabled (default: true)." }
            },
            "required": ["description", "schedule"],
            "additionalProperties": false
        }),
    )
}

use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "cron_create",
        "Create a cron/scheduled job record (persisted).",
        json!({
            "type": "object",
            "properties": {
                "job_id": { "type": "string", "description": "Optional stable job id. If omitted, LoopForge generates one." },
                "name": { "type": "string", "description": "Job name." },
                "schedule": { "type": "object", "description": "Schedule payload (stored as-is)." },
                "action": { "type": "object", "description": "Action payload (stored as-is)." },
                "delivery": { "type": "object", "description": "Optional delivery payload (stored as-is)." },
                "one_shot": { "type": "boolean", "description": "If true, job should be considered one-shot (stored)." },
                "enabled": { "type": "boolean", "description": "Whether this job is enabled (default: true)." }
            },
            "required": ["name", "schedule", "action"],
            "additionalProperties": false
        }),
    )
}

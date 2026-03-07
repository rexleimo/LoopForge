use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "cron_cancel",
        "Cancel a cron/scheduled job record by id.",
        json!({
            "type": "object",
            "properties": {
                "job_id": { "type": "string", "description": "Job id." }
            },
            "required": ["job_id"],
            "additionalProperties": false
        }),
    )
}

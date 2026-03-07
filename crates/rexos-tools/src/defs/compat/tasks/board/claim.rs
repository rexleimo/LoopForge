use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn task_claim_def() -> ToolDefinition {
    function_def(
        "task_claim",
        "Claim the next available pending task.",
        json!({
            "type": "object",
            "properties": {
                "agent_id": { "type": "string", "description": "Optional agent id claiming the task." }
            },
            "additionalProperties": false
        }),
    )
}

use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::function_def;

pub(super) fn workflow_tool_defs() -> Vec<ToolDefinition> {
    vec![workflow_run_def()]
}

fn workflow_run_def() -> ToolDefinition {
    function_def(
        "workflow_run",
        "Run a persisted multi-step workflow and save execution state under .loopforge/workflows/.",
        json!({
            "type": "object",
            "properties": {
                "workflow_id": { "type": "string", "description": "Optional stable workflow id. If omitted, LoopForge generates one." },
                "name": { "type": "string", "description": "Optional workflow display name." },
                "continue_on_error": { "type": "boolean", "description": "Whether to continue executing remaining steps after a failed step (default false)." },
                "steps": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string", "description": "Optional step name." },
                            "tool": { "type": "string", "description": "Tool name to execute." },
                            "arguments": { "type": "object", "description": "Tool arguments JSON object." },
                            "approval_required": { "type": "boolean", "description": "Force approval gate for this step when approval mode is enabled." }
                        },
                        "required": ["tool"],
                        "additionalProperties": false
                    }
                }
            },
            "required": ["steps"],
            "additionalProperties": false
        }),
    )
}

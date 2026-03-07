use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::function_def;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![docker_exec_def()]
}

fn docker_exec_def() -> ToolDefinition {
    function_def(
        "docker_exec",
        "Run a command inside a one-shot Docker container with the workspace mounted (disabled by default).",
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Command to execute inside the container (passed to `sh -lc`)." }
            },
            "required": ["command"],
            "additionalProperties": false
        }),
    )
}

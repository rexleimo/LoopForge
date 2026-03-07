use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::function_def;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![shell_exec_def()]
}

fn shell_exec_def() -> ToolDefinition {
    function_def(
        "shell_exec",
        "Execute a shell command and return its output.",
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "The command to execute" },
                "timeout_seconds": { "type": "integer", "description": "Timeout in seconds (default: 30)" }
            },
            "required": ["command"],
            "additionalProperties": false
        }),
    )
}

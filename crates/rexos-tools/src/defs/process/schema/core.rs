use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::function_def;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    vec![shell_def()]
}

fn shell_def() -> ToolDefinition {
    function_def(
        "shell",
        "Run a shell command inside the workspace (bash on Unix, PowerShell on Windows).",
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Command to run." },
                "timeout_ms": { "type": "integer", "description": "Timeout in milliseconds (default 60000).", "minimum": 1 }
            },
            "required": ["command"],
            "additionalProperties": false
        }),
    )
}

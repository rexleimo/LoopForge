use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn process_start_def() -> ToolDefinition {
    function_def(
        "process_start",
        "Start a long-running process (REPL/server). Returns a process_id.",
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Executable to run (e.g. 'python', 'node', 'bash')." },
                "args": { "type": "array", "items": { "type": "string" }, "description": "Optional command-line args." }
            },
            "required": ["command"],
            "additionalProperties": false
        }),
    )
}

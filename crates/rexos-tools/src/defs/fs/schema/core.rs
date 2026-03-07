use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::function_def;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    vec![fs_read_def(), fs_write_def()]
}

fn fs_read_def() -> ToolDefinition {
    function_def(
        "fs_read",
        "Read a UTF-8 text file from the workspace.",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Relative path inside the workspace." }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}

fn fs_write_def() -> ToolDefinition {
    function_def(
        "fs_write",
        "Write a UTF-8 text file to the workspace (creates parent dirs).",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Relative path inside the workspace." },
                "content": { "type": "string", "description": "Full file contents to write." }
            },
            "required": ["path", "content"],
            "additionalProperties": false
        }),
    )
}

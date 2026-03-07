use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn task_post_def() -> ToolDefinition {
    function_def(
        "task_post",
        "Post a task into the shared task board.",
        json!({
            "type": "object",
            "properties": {
                "task_id": { "type": "string", "description": "Optional stable task id. If omitted, LoopForge generates one." },
                "title": { "type": "string", "description": "Short title." },
                "description": { "type": "string", "description": "Task description." },
                "assigned_to": { "type": "string", "description": "Optional assignee agent id." }
            },
            "required": ["title", "description"],
            "additionalProperties": false
        }),
    )
}

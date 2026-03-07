use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};

pub(super) fn browser_back_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_back".to_string(),
            description: "Go back in browser history.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        },
    }
}

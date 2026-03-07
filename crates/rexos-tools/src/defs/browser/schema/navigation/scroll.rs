use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};

pub(super) fn browser_scroll_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_scroll".to_string(),
            description: "Scroll the current page.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "direction": { "type": "string", "description": "Scroll direction: down/up/left/right (default down).", "enum": ["down", "up", "left", "right"] },
                    "amount": { "type": "integer", "description": "Scroll amount in pixels (default 600).", "minimum": 0 }
                },
                "required": [],
                "additionalProperties": false
            }),
        },
    }
}

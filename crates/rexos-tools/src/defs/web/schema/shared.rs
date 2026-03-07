use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};
use serde_json::Value;

pub(super) fn function_def(name: &str, description: &str, parameters: Value) -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        },
    }
}

pub(super) fn pdf_parameters() -> Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "path": { "type": "string", "description": "Relative path to a .pdf file in the workspace." },
            "pages": { "type": "string", "description": "Optional page selector like '1' or '1,3-5'." },
            "max_pages": { "type": "integer", "description": "Maximum pages to return (default 10).", "minimum": 1 },
            "max_chars": { "type": "integer", "description": "Maximum characters to return (default 12000).", "minimum": 1 }
        },
        "required": ["path"],
        "additionalProperties": false
    })
}

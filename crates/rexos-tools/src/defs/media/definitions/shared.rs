use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};
use serde_json::Value;

pub(super) fn compat_function(name: &str, description: &str, parameters: Value) -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        },
    }
}

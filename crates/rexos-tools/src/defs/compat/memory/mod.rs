mod recall;
mod store;
#[cfg(test)]
mod tests;

use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};
use serde_json::Value;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![store::tool_def(), recall::tool_def()]
}

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

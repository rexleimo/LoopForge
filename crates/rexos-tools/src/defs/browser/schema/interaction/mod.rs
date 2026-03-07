mod input;
mod script;
mod shared;
mod wait;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(input::tool_defs());
    defs.extend(wait::tool_defs());
    defs.extend(script::tool_defs());
    defs
}

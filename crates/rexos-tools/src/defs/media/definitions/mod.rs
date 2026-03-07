mod analyze;
mod generate;
mod shared;
mod transcribe;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    Vec::new()
}

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(analyze::tool_defs());
    defs.extend(transcribe::tool_defs());
    defs.extend(generate::tool_defs());
    defs
}

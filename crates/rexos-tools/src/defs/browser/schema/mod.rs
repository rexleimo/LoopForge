mod interaction;
mod navigation;
mod readback;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(navigation::tool_defs());
    defs.extend(interaction::tool_defs());
    defs.extend(readback::tool_defs());
    defs
}

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    Vec::new()
}

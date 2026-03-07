mod container;
mod lifecycle;
mod shared;
mod shell;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(shell::tool_defs());
    defs.extend(container::tool_defs());
    defs.extend(lifecycle::tool_defs());
    defs
}

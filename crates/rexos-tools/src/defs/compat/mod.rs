mod agents;
mod hands;
mod knowledge;
mod memory;
mod scheduling;
mod tasks;
#[cfg(test)]
mod tests;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(memory::compat_tool_defs());
    defs.extend(agents::compat_tool_defs());
    defs.extend(tasks::compat_tool_defs());
    defs.extend(scheduling::compat_tool_defs());
    defs.extend(knowledge::compat_tool_defs());
    defs.extend(hands::compat_tool_defs());
    defs
}

mod cron;
mod schedule;
mod shared;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(schedule::tool_defs());
    defs.extend(cron::tool_defs());
    defs
}

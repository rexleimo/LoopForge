mod board;
mod events;
mod shared;
mod workflow;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(board::task_tool_defs());
    defs.extend(events::event_tool_defs());
    defs.extend(workflow::workflow_tool_defs());
    defs
}

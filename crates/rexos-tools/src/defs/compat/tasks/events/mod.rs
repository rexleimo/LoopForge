mod channel;
mod publish;
#[cfg(test)]
mod tests;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn event_tool_defs() -> Vec<ToolDefinition> {
    vec![event_publish_def(), channel_send_def()]
}

fn event_publish_def() -> ToolDefinition {
    publish::tool_def()
}

fn channel_send_def() -> ToolDefinition {
    channel::tool_def()
}

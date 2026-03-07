mod cancel;
mod create;
mod list;
#[cfg(test)]
mod tests;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        cron_create_schema_def(),
        cron_list_schema_def(),
        cron_cancel_schema_def(),
    ]
}

fn cron_create_schema_def() -> ToolDefinition {
    create::tool_def()
}

fn cron_list_schema_def() -> ToolDefinition {
    list::tool_def()
}

fn cron_cancel_schema_def() -> ToolDefinition {
    cancel::tool_def()
}

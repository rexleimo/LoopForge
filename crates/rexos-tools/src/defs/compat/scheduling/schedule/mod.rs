mod create;
mod delete;
mod list;
#[cfg(test)]
mod tests;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        schedule_create_schema_def(),
        schedule_list_schema_def(),
        schedule_delete_schema_def(),
    ]
}

fn schedule_create_schema_def() -> ToolDefinition {
    create::tool_def()
}

fn schedule_list_schema_def() -> ToolDefinition {
    list::tool_def()
}

fn schedule_delete_schema_def() -> ToolDefinition {
    delete::tool_def()
}

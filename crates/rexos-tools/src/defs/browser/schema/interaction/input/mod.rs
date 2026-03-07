mod click;
mod press_key;
#[cfg(test)]
mod tests;
mod type_text;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        browser_click_schema_def(),
        browser_type_schema_def(),
        browser_press_key_schema_def(),
    ]
}

fn browser_click_schema_def() -> ToolDefinition {
    click::tool_def()
}

fn browser_type_schema_def() -> ToolDefinition {
    type_text::tool_def()
}

fn browser_press_key_schema_def() -> ToolDefinition {
    press_key::tool_def()
}

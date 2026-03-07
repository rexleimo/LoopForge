mod classify;
mod field;
mod route;
mod script;
mod scroll;
#[cfg(test)]
mod tests;
mod wait;

use crate::Toolset;

fn is_field_input_tool(name: &str) -> bool {
    classify::is_field_input_tool(name)
}

fn is_wait_input_tool(name: &str) -> bool {
    classify::is_wait_input_tool(name)
}

fn is_scroll_input_tool(name: &str) -> bool {
    classify::is_scroll_input_tool(name)
}

fn is_script_input_tool(name: &str) -> bool {
    classify::is_script_input_tool(name)
}

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    route::dispatch(toolset, name, arguments_json).await
}

use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    if super::is_field_input_tool(name) {
        return super::field::dispatch(toolset, name, arguments_json).await;
    }
    if super::is_wait_input_tool(name) {
        return super::wait::dispatch(toolset, name, arguments_json).await;
    }
    if super::is_scroll_input_tool(name) {
        return super::scroll::dispatch(toolset, name, arguments_json).await;
    }
    if super::is_script_input_tool(name) {
        return super::script::dispatch(toolset, name, arguments_json).await;
    }
    unreachable!("unexpected browser input tool: {name}")
}

use crate::Toolset;

pub(super) async fn dispatch(toolset: &Toolset, arguments_json: &str) -> anyhow::Result<String> {
    let _args: serde_json::Value = super::super::parse_args(arguments_json, "location_get")?;
    toolset.location_get()
}

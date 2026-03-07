use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "process_list" => {
            let _args: serde_json::Value = super::super::parse_args(arguments_json, name)?;
            toolset.process_list().await
        }
        _ => unreachable!("unexpected process list tool: {name}"),
    }
}

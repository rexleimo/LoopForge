use crate::defs::BrowserScrollArgs;
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_scroll" => {
            let args: BrowserScrollArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset
                .browser_scroll(args.direction.as_deref(), args.amount)
                .await
        }
        _ => unreachable!("unexpected browser scroll input tool: {name}"),
    }
}

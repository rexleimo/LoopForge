use crate::defs::BrowserRunJsArgs;
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_run_js" => {
            let args: BrowserRunJsArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset.browser_run_js(&args.expression).await
        }
        _ => unreachable!("unexpected browser script input tool: {name}"),
    }
}

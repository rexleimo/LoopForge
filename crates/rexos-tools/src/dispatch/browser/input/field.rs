use crate::defs::{BrowserClickArgs, BrowserPressKeyArgs, BrowserTypeArgs};
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_click" => {
            let args: BrowserClickArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset.browser_click(&args.selector).await
        }
        "browser_type" => {
            let args: BrowserTypeArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset.browser_type(&args.selector, &args.text).await
        }
        "browser_press_key" => {
            let args: BrowserPressKeyArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset
                .browser_press_key(args.selector.as_deref(), &args.key)
                .await
        }
        _ => unreachable!("unexpected browser field input tool: {name}"),
    }
}

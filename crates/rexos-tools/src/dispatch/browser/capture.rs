use crate::defs::BrowserScreenshotArgs;
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_read_page" => {
            let _args: serde_json::Value =
                super::super::parse_args(arguments_json, "browser_read_page")?;
            toolset.browser_read_page().await
        }
        "browser_screenshot" => {
            let args: BrowserScreenshotArgs =
                super::super::parse_args(arguments_json, "browser_screenshot")?;
            toolset.browser_screenshot(args.path.as_deref()).await
        }
        _ => unreachable!("unexpected browser capture tool: {name}"),
    }
}

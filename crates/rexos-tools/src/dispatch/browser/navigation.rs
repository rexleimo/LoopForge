use crate::defs::BrowserNavigateArgs;
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_navigate" => {
            let args: BrowserNavigateArgs =
                super::super::parse_args(arguments_json, "browser_navigate")?;
            toolset
                .browser_navigate(
                    &args.url,
                    args.timeout_ms,
                    args.allow_private,
                    args.headless,
                )
                .await
        }
        "browser_back" => {
            let _args: serde_json::Value =
                super::super::parse_args(arguments_json, "browser_back")?;
            toolset.browser_back().await
        }
        "browser_close" => {
            let _args: serde_json::Value =
                super::super::parse_args(arguments_json, "browser_close")?;
            toolset.browser_close().await
        }
        _ => unreachable!("unexpected browser navigation tool: {name}"),
    }
}

use crate::defs::{BrowserWaitArgs, BrowserWaitForArgs};
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "browser_wait" => {
            let args: BrowserWaitArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset.browser_wait(&args.selector, args.timeout_ms).await
        }
        "browser_wait_for" => {
            let args: BrowserWaitForArgs = super::super::super::parse_args(arguments_json, name)?;
            toolset
                .browser_wait_for(
                    args.selector.as_deref(),
                    args.text.as_deref(),
                    args.timeout_ms,
                )
                .await
        }
        _ => unreachable!("unexpected browser wait input tool: {name}"),
    }
}

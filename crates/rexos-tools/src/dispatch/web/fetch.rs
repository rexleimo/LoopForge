use crate::defs::WebFetchArgs;
use crate::Toolset;

pub(super) async fn dispatch(toolset: &Toolset, arguments_json: &str) -> anyhow::Result<String> {
    let args: WebFetchArgs = super::super::parse_args(arguments_json, "web_fetch")?;
    toolset
        .web_fetch(
            &args.url,
            args.timeout_ms,
            args.max_bytes,
            args.allow_private,
        )
        .await
}

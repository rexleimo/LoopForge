use crate::defs::WebSearchArgs;
use crate::Toolset;

pub(super) async fn dispatch(toolset: &Toolset, arguments_json: &str) -> anyhow::Result<String> {
    let args: WebSearchArgs = super::super::parse_args(arguments_json, "web_search")?;
    toolset.web_search(&args.query, args.max_results).await
}

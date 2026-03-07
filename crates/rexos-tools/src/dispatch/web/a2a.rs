use anyhow::Context;

use crate::defs::{A2aDiscoverArgs, A2aSendArgs};
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "a2a_discover" => {
            let args: A2aDiscoverArgs = super::super::parse_args(arguments_json, "a2a_discover")?;
            toolset.a2a_discover(&args.url, args.allow_private).await
        }
        "a2a_send" => {
            let args: A2aSendArgs = super::super::parse_args(arguments_json, "a2a_send")?;
            let url = args
                .agent_url
                .as_deref()
                .or(args.url.as_deref())
                .context("missing agent_url (or url) for a2a_send")?;
            toolset
                .a2a_send(
                    url,
                    &args.message,
                    args.session_id.as_deref(),
                    args.allow_private,
                )
                .await
        }
        _ => unreachable!("unexpected a2a tool: {name}"),
    }
}

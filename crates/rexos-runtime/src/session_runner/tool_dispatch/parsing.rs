use anyhow::Context;
use serde::de::DeserializeOwned;

pub(super) fn parse_args<T>(args_json: &str, tool_name: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(args_json).with_context(|| format!("parse {tool_name} args"))
}

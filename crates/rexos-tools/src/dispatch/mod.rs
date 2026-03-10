mod browser;
mod domain;
mod fs;
mod mcp;
mod media;
mod process;
mod routing;
mod web;

use anyhow::Context;
use serde::de::DeserializeOwned;

pub(crate) use domain::{tool_call_domain, ToolCallDomain};

fn parse_args<T: DeserializeOwned>(arguments_json: &str, tool_name: &str) -> anyhow::Result<T> {
    serde_json::from_str(arguments_json).with_context(|| format!("parse {tool_name} arguments"))
}

use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::dispatch::parse_args;
use crate::Toolset;

#[derive(Debug, Default, Deserialize)]
struct McpListArgs {
    #[serde(default)]
    server: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct McpReadArgs {
    #[serde(default)]
    server: Option<String>,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct McpPromptGetArgs {
    #[serde(default)]
    server: Option<String>,
    name: String,
    #[serde(default)]
    arguments: Option<Value>,
}

impl Toolset {
    pub(super) async fn call_mcp_tool(
        &self,
        name: &str,
        arguments_json: &str,
    ) -> anyhow::Result<String> {
        let hub = self
            .mcp
            .as_ref()
            .ok_or_else(|| anyhow!("mcp is not enabled for this session (pass --mcp-config)"))?;

        let value = match name {
            "mcp_servers_list" => {
                Value::Array(hub.server_names().into_iter().map(Value::String).collect())
            }
            "mcp_resources_list" => {
                let args: McpListArgs = parse_args(arguments_json, name)?;
                hub.resources_list(args.server.as_deref(), args.cursor.as_deref())
                    .await?
            }
            "mcp_resources_read" => {
                let args: McpReadArgs = parse_args(arguments_json, name)?;
                hub.resources_read(args.server.as_deref(), &args.uri)
                    .await?
            }
            "mcp_prompts_list" => {
                let args: McpListArgs = parse_args(arguments_json, name)?;
                hub.prompts_list(args.server.as_deref(), args.cursor.as_deref())
                    .await?
            }
            "mcp_prompts_get" => {
                let args: McpPromptGetArgs = parse_args(arguments_json, name)?;
                hub.prompts_get(args.server.as_deref(), &args.name, args.arguments)
                    .await?
            }
            _ => {
                let args = parse_json_value_or_empty_object(arguments_json, name)?;
                hub.call_tool(name, args).await?
            }
        };

        serde_json::to_string(&value).context("serialize mcp tool result")
    }
}

fn parse_json_value_or_empty_object(
    arguments_json: &str,
    tool_name: &str,
) -> anyhow::Result<Value> {
    let raw = arguments_json.trim();
    if raw.is_empty() || raw == "null" {
        return Ok(Value::Object(Map::new()));
    }
    serde_json::from_str(raw).with_context(|| format!("parse {tool_name} arguments"))
}

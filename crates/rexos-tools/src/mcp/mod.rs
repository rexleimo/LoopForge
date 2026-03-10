mod config;
mod jsonrpc;
mod stdio;
mod types;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};
use serde_json::Value;

pub(crate) use config::{McpServerConfig, McpServersConfig};
use jsonrpc::JsonRpcClient;
use stdio::{spawn_stdio_server, StdioServer};
use types::ToolsListResult;

#[derive(Debug, Clone)]
pub(crate) struct McpHub {
    servers: BTreeMap<String, Arc<McpServer>>,
    tool_targets: HashMap<String, McpToolTarget>,
    tool_defs: Vec<ToolDefinition>,
}

#[derive(Debug)]
struct McpServer {
    #[allow(dead_code)]
    name: String,
    stdio: StdioServer,
}

#[derive(Debug, Clone)]
struct McpToolTarget {
    server: String,
    remote_name: String,
}

impl McpHub {
    pub(crate) async fn connect_from_json(
        config_json: &str,
        workspace_root: &Path,
    ) -> anyhow::Result<Self> {
        let cfg: McpServersConfig =
            serde_json::from_str(config_json).context("parse mcp servers config JSON")?;
        Self::connect(cfg, workspace_root).await
    }

    pub(crate) async fn connect(
        cfg: McpServersConfig,
        workspace_root: &Path,
    ) -> anyhow::Result<Self> {
        if cfg.servers.is_empty() {
            return Err(anyhow!("mcp config has no servers"));
        }

        let mut servers: BTreeMap<String, Arc<McpServer>> = BTreeMap::new();
        let mut tool_targets: HashMap<String, McpToolTarget> = HashMap::new();
        let mut tool_defs: Vec<ToolDefinition> = Vec::new();
        let mut used_names: HashSet<String> = HashSet::new();

        for (name, server_cfg) in &cfg.servers {
            let stdio = spawn_stdio_server(name, server_cfg, workspace_root).await?;
            initialize(&stdio.client)
                .await
                .with_context(|| format!("mcp initialize: {name}"))?;
            let server = Arc::new(McpServer {
                name: name.clone(),
                stdio,
            });

            let tools = list_all_tools(&server.stdio.client)
                .await
                .with_context(|| format!("mcp tools/list: {name}"))?;

            for tool in tools {
                let local = allocate_local_tool_name(name, &tool.name, &mut used_names);
                tool_targets.insert(
                    local.clone(),
                    McpToolTarget {
                        server: name.clone(),
                        remote_name: tool.name.clone(),
                    },
                );

                tool_defs.push(ToolDefinition {
                    kind: "function".to_string(),
                    function: ToolFunctionDefinition {
                        name: local,
                        description: tool
                            .description
                            .unwrap_or_else(|| format!("MCP tool '{name}::{}'", tool.name)),
                        parameters: if tool.input_schema.is_null() {
                            serde_json::json!({ "type": "object" })
                        } else {
                            tool.input_schema
                        },
                    },
                });
            }

            servers.insert(name.clone(), server);
        }

        Ok(Self {
            servers,
            tool_targets,
            tool_defs,
        })
    }

    pub(crate) fn tool_definitions(&self) -> &[ToolDefinition] {
        &self.tool_defs
    }

    pub(crate) fn server_names(&self) -> Vec<String> {
        self.servers.keys().cloned().collect()
    }

    pub(crate) async fn call_tool(
        &self,
        local_name: &str,
        arguments: Value,
    ) -> anyhow::Result<Value> {
        let target = self
            .tool_targets
            .get(local_name)
            .ok_or_else(|| anyhow!("unknown mcp tool: {local_name}"))?
            .clone();

        let server = self
            .servers
            .get(&target.server)
            .ok_or_else(|| anyhow!("unknown mcp server: {}", target.server))?;

        server
            .stdio
            .client
            .request(
                "tools/call",
                Some(serde_json::json!({
                    "name": target.remote_name,
                    "arguments": arguments,
                })),
            )
            .await
    }

    pub(crate) async fn resources_list(
        &self,
        server: Option<&str>,
        cursor: Option<&str>,
    ) -> anyhow::Result<Value> {
        self.forward_list_request("resources/list", "resources", server, cursor)
            .await
    }

    pub(crate) async fn prompts_list(
        &self,
        server: Option<&str>,
        cursor: Option<&str>,
    ) -> anyhow::Result<Value> {
        self.forward_list_request("prompts/list", "prompts", server, cursor)
            .await
    }

    async fn forward_list_request(
        &self,
        method: &str,
        field: &str,
        server: Option<&str>,
        cursor: Option<&str>,
    ) -> anyhow::Result<Value> {
        let cursor = cursor.map(|c| c.trim()).filter(|c| !c.is_empty());
        let params = cursor.map(|cursor| serde_json::json!({ "cursor": cursor }));

        match server.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            Some(name) => {
                let server = self
                    .servers
                    .get(name)
                    .ok_or_else(|| anyhow!("unknown mcp server: {name}"))?;
                let result = server.stdio.client.request(method, params).await?;
                Ok(serde_json::json!({ "server": name, "result": result }))
            }
            None => {
                let mut all: Vec<Value> = Vec::new();
                for name in self.servers.keys() {
                    let server = self
                        .servers
                        .get(name)
                        .ok_or_else(|| anyhow!("unknown mcp server: {name}"))?;
                    let result = server.stdio.client.request(method, params.clone()).await?;
                    let items = result.get(field).cloned().unwrap_or(Value::Null);
                    all.push(serde_json::json!({ "server": name, field: items, "nextCursor": result.get("nextCursor") }));
                }
                Ok(Value::Array(all))
            }
        }
    }

    pub(crate) async fn resources_read(
        &self,
        server: Option<&str>,
        uri: &str,
    ) -> anyhow::Result<Value> {
        let uri = uri.trim();
        if uri.is_empty() {
            return Err(anyhow!("mcp_resources_read: uri is empty"));
        }

        match server.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            Some(name) => {
                let server = self
                    .servers
                    .get(name)
                    .ok_or_else(|| anyhow!("unknown mcp server: {name}"))?;
                let result = server
                    .stdio
                    .client
                    .request("resources/read", Some(serde_json::json!({ "uri": uri })))
                    .await?;
                Ok(serde_json::json!({ "server": name, "result": result }))
            }
            None => {
                for name in self.servers.keys() {
                    let server = self
                        .servers
                        .get(name)
                        .ok_or_else(|| anyhow!("unknown mcp server: {name}"))?;
                    let res = server
                        .stdio
                        .client
                        .request("resources/read", Some(serde_json::json!({ "uri": uri })))
                        .await;
                    if let Ok(result) = res {
                        return Ok(serde_json::json!({ "server": name, "result": result }));
                    }
                }
                Err(anyhow!("mcp_resources_read: no server handled uri: {uri}"))
            }
        }
    }

    pub(crate) async fn prompts_get(
        &self,
        server: Option<&str>,
        name: &str,
        arguments: Option<Value>,
    ) -> anyhow::Result<Value> {
        let name = name.trim();
        if name.is_empty() {
            return Err(anyhow!("mcp_prompts_get: name is empty"));
        }

        let mut params = serde_json::Map::new();
        params.insert("name".to_string(), Value::String(name.to_string()));
        if let Some(arguments) = arguments {
            params.insert("arguments".to_string(), arguments);
        }
        let params = Value::Object(params);

        match server.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            Some(server_name) => {
                let server = self
                    .servers
                    .get(server_name)
                    .ok_or_else(|| anyhow!("unknown mcp server: {server_name}"))?;
                let result = server
                    .stdio
                    .client
                    .request("prompts/get", Some(params))
                    .await?;
                Ok(serde_json::json!({ "server": server_name, "result": result }))
            }
            None => {
                for server_name in self.servers.keys() {
                    let server = self
                        .servers
                        .get(server_name)
                        .ok_or_else(|| anyhow!("unknown mcp server: {server_name}"))?;
                    let res = server
                        .stdio
                        .client
                        .request("prompts/get", Some(params.clone()))
                        .await;
                    if let Ok(result) = res {
                        return Ok(serde_json::json!({ "server": server_name, "result": result }));
                    }
                }
                Err(anyhow!("mcp_prompts_get: no server handled prompt: {name}"))
            }
        }
    }
}

async fn initialize(client: &JsonRpcClient) -> anyhow::Result<()> {
    // Try a small set of known protocol revisions (latest-first) for broad compatibility.
    const VERSIONS: [&str; 3] = ["2025-11-25", "2025-03-26", "2024-11-05"];

    let params_base = |protocol: &str| {
        serde_json::json!({
            "protocolVersion": protocol,
            "capabilities": {},
            "clientInfo": {
                "name": "loopforge",
                "version": env!("CARGO_PKG_VERSION"),
            }
        })
    };

    let mut last_err: Option<anyhow::Error> = None;
    for v in VERSIONS {
        match client.request("initialize", Some(params_base(v))).await {
            Ok(_) => {
                client.notify("initialized", None).await?;
                return Ok(());
            }
            Err(err) => {
                last_err = Some(err);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| anyhow!("mcp initialize failed")))
}

async fn list_all_tools(client: &JsonRpcClient) -> anyhow::Result<Vec<types::McpTool>> {
    let mut cursor: Option<String> = None;
    let mut out: Vec<types::McpTool> = Vec::new();
    for _ in 0..32usize {
        let params = cursor
            .as_deref()
            .map(|cursor| serde_json::json!({ "cursor": cursor }));
        let value = client.request("tools/list", params).await?;
        let parsed: ToolsListResult =
            serde_json::from_value(value).context("decode tools/list result")?;
        out.extend(parsed.tools.into_iter());
        cursor = parsed.next_cursor;
        if cursor.as_deref().unwrap_or("").trim().is_empty() {
            break;
        }
    }
    Ok(out)
}

fn allocate_local_tool_name(server: &str, tool: &str, used: &mut HashSet<String>) -> String {
    let server_part = sanitize_component(server);
    let tool_part = sanitize_component(tool);
    let mut candidate = format!("mcp_{server_part}__{tool_part}");

    if candidate.len() > 64 {
        let hash = short_hash(&candidate);
        let suffix = format!("_{hash:08x}");
        candidate.truncate(64usize.saturating_sub(suffix.len()));
        candidate.push_str(&suffix);
    }

    if used.insert(candidate.clone()) {
        return candidate;
    }

    let hash = short_hash(&format!("{server}\0{tool}"));
    let suffix = format!("_{hash:08x}");
    let mut out = candidate;
    if out.len() + suffix.len() > 64 {
        out.truncate(64usize.saturating_sub(suffix.len()));
    }
    out.push_str(&suffix);
    used.insert(out.clone());
    out
}

fn sanitize_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        let c = if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            c.to_ascii_lowercase()
        } else {
            '_'
        };
        out.push(c);
    }
    out.trim_matches('_').to_string()
}

fn short_hash(value: &str) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

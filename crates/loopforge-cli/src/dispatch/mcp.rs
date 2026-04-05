use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use serde_json::Value;

use super::mcp_sanitize::sanitize_mcp_config;
use crate::{cli::McpCommand, runtime_env};

pub(super) async fn run(command: McpCommand) -> anyhow::Result<()> {
    match command {
        McpCommand::Diagnose {
            workspace,
            session,
            config,
            resources,
            prompts,
            json,
        } => diagnose(workspace, session, config, resources, prompts, json).await,
    }
}

async fn diagnose(
    workspace: PathBuf,
    session: Option<String>,
    config: Option<PathBuf>,
    resources: bool,
    prompts: bool,
    json: bool,
) -> anyhow::Result<()> {
    let (_paths, agent) = runtime_env::load_agent_runtime()?;
    let (_paths, cfg) = runtime_env::load_runtime_config()?;

    std::fs::create_dir_all(&workspace)
        .with_context(|| format!("create workspace: {}", workspace.display()))?;

    let session_id = match session {
        Some(id) => id,
        None => rexos::harness::resolve_session_id(&workspace)?,
    };

    let raw_config = match config.as_ref() {
        Some(path) => {
            let raw = std::fs::read_to_string(path)
                .with_context(|| format!("read mcp config: {}", path.display()))?;
            normalize_json_string(&raw).context("normalize mcp config json")?
        }
        None => {
            let snapshot = agent
                .load_session_policy_snapshot(&session_id)
                .with_context(|| format!("load session policy snapshot: {session_id}"))?;
            snapshot
                .mcp_config_json
                .ok_or_else(|| anyhow!("no MCP config is set for session {session_id}"))?
        }
    };

    let parsed_config: Value =
        serde_json::from_str(&raw_config).context("parse mcp config JSON")?;
    let sanitized_config = sanitize_mcp_config(&parsed_config);

    let mut tools =
        rexos::tools::Toolset::new_with_security_config(workspace.clone(), cfg.security.clone())?;
    tools
        .enable_mcp_from_json(&raw_config)
        .await
        .context("connect mcp servers")?;

    let servers_raw = tools.call("mcp_servers_list", r#"{}"#).await?;
    let servers_json: Value =
        serde_json::from_str(&servers_raw).context("decode mcp_servers_list output")?;
    let tool_names = list_remote_tool_names(&tools);

    let resources_json = if resources {
        let out = tools.call("mcp_resources_list", r#"{}"#).await?;
        Some(serde_json::from_str(&out).context("decode mcp_resources_list output")?)
    } else {
        None
    };

    let prompts_json = if prompts {
        let out = tools.call("mcp_prompts_list", r#"{}"#).await?;
        Some(serde_json::from_str(&out).context("decode mcp_prompts_list output")?)
    } else {
        None
    };

    if json {
        let out = build_mcp_diagnose_json(
            &workspace,
            &session_id,
            sanitized_config,
            servers_json,
            tool_names,
            resources_json,
            prompts_json,
        )?;
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("MCP diagnose");
    println!();
    println!("workspace: {}", workspace.display());
    println!("session_id: {session_id}");
    if let Some(path) = config.as_ref() {
        println!("config_source: file {}", path.display());
    } else {
        println!("config_source: session");
    }
    if let Some(servers) = sanitized_config
        .get("servers")
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().cloned().collect::<Vec<_>>())
    {
        if !servers.is_empty() {
            println!("config_servers: {}", servers.join(", "));
        }
    }
    println!();
    println!("servers: {}", servers_raw.trim());
    println!("remote_tools: {} tool(s)", tool_names.len());
    for name in tool_names {
        println!("- {name}");
    }
    if let Some(v) = resources_json {
        println!();
        println!("resources_list: {}", serde_json::to_string_pretty(&v)?);
    }
    if let Some(v) = prompts_json {
        println!();
        println!("prompts_list: {}", serde_json::to_string_pretty(&v)?);
    }
    Ok(())
}

fn build_mcp_diagnose_json(
    workspace: &PathBuf,
    session_id: &str,
    sanitized_config: Value,
    servers_json: Value,
    tool_names: Vec<String>,
    resources_json: Option<Value>,
    prompts_json: Option<Value>,
) -> anyhow::Result<Value> {
    let mut out = BTreeMap::new();
    out.insert(
        "workspace".to_string(),
        Value::String(workspace.display().to_string()),
    );
    out.insert(
        "session_id".to_string(),
        Value::String(session_id.to_string()),
    );
    out.insert("config".to_string(), sanitized_config);
    out.insert("servers".to_string(), servers_json);
    out.insert("tool_names".to_string(), serde_json::to_value(tool_names)?);
    if let Some(v) = resources_json {
        out.insert("resources".to_string(), v);
    }
    if let Some(v) = prompts_json {
        out.insert("prompts".to_string(), v);
    }
    Ok(Value::Object(out.into_iter().collect()))
}

fn normalize_json_string(raw: &str) -> anyhow::Result<String> {
    let json: Value = serde_json::from_str(raw).context("parse JSON")?;
    Ok(serde_json::to_string(&json)?)
}

fn list_remote_tool_names(tools: &rexos::tools::Toolset) -> Vec<String> {
    let mut names: Vec<String> = tools
        .definitions()
        .into_iter()
        .filter_map(|def| {
            let name = def.function.name;
            if name.starts_with("mcp_") && name.contains("__") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_mcp_diagnose_json_includes_expected_keys_and_redaction() {
        let tmp = tempfile::tempdir().unwrap();
        let workspace = tmp.path().join("workspace");

        let config = serde_json::json!({
            "servers": {
                "s1": {
                    "command": "python",
                    "env": { "API_KEY": "secret" },
                }
            }
        });
        let sanitized = sanitize_mcp_config(&config);

        let out = build_mcp_diagnose_json(
            &workspace,
            "s-test",
            sanitized,
            serde_json::json!(["s1"]),
            vec!["mcp_s1__echo".to_string()],
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            out["workspace"].as_str(),
            Some(workspace.display().to_string().as_str())
        );
        assert_eq!(out["session_id"].as_str(), Some("s-test"));
        assert!(out.get("servers").is_some());
        assert!(out.get("tool_names").is_some());
        assert_eq!(
            out["config"]["servers"]["s1"]["env"]["API_KEY"].as_str(),
            Some("[redacted]")
        );

        let tool_names: Vec<String> = out["tool_names"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        assert_eq!(tool_names, vec!["mcp_s1__echo".to_string()]);
        assert!(out.get("resources").is_none());
        assert!(out.get("prompts").is_none());
    }

    #[test]
    fn normalize_json_string_round_trips() {
        let out = normalize_json_string(" {\"servers\":{}} ").unwrap();
        assert_eq!(out, "{\"servers\":{}}");
    }

    #[test]
    fn sanitize_mcp_config_redacts_env_values() {
        let input = serde_json::json!({
            "servers": {
                "s1": {
                    "env": { "API_KEY": "secret" },
                    "command": "python"
                }
            }
        });
        let sanitized = sanitize_mcp_config(&input);
        assert_eq!(
            sanitized["servers"]["s1"]["env"]["API_KEY"].as_str(),
            Some("[redacted]")
        );
        assert_eq!(
            sanitized["servers"]["s1"]["command"].as_str(),
            Some("python")
        );
    }
}

use std::collections::BTreeMap;

use anyhow::Context;
use serde_json::Value;

use crate::{cli::SessionCommand, runtime_env};

pub(super) async fn run(command: SessionCommand) -> anyhow::Result<()> {
    match command {
        SessionCommand::Policy {
            workspace,
            session,
            show_mcp_config,
            json,
        } => {
            let (_paths, agent) = runtime_env::load_agent_runtime()?;

            let session_id = match session {
                Some(id) => id,
                None => rexos::harness::resolve_session_id(&workspace)?,
            };

            let snapshot = agent
                .load_session_policy_snapshot(&session_id)
                .with_context(|| format!("load session policy snapshot: {session_id}"))?;

            let (mcp_servers, mcp_config_sanitized, mcp_parse_error) =
                summarize_mcp_config(snapshot.mcp_config_json.as_deref(), show_mcp_config);

            if json {
                let mut out = BTreeMap::new();
                out.insert("session_id".to_string(), Value::String(session_id));
                out.insert(
                    "allowed_tools".to_string(),
                    serde_json::to_value(&snapshot.allowed_tools)?,
                );
                out.insert(
                    "allowed_skills".to_string(),
                    serde_json::to_value(&snapshot.allowed_skills)?,
                );
                out.insert(
                    "skill_policy".to_string(),
                    serde_json::to_value(&snapshot.skill_policy)?,
                );

                let mut mcp = BTreeMap::new();
                mcp.insert(
                    "present".to_string(),
                    Value::Bool(snapshot.mcp_config_json.is_some()),
                );
                if let Some(servers) = mcp_servers {
                    mcp.insert("servers".to_string(), serde_json::to_value(servers)?);
                }
                if let Some(err) = mcp_parse_error {
                    mcp.insert("parse_error".to_string(), Value::String(err));
                }
                if let Some(cfg) = mcp_config_sanitized {
                    mcp.insert("config".to_string(), cfg);
                }
                out.insert("mcp".to_string(), Value::Object(mcp.into_iter().collect()));

                println!("{}", serde_json::to_string_pretty(&out)?);
                return Ok(());
            }

            println!("Session policy");
            println!();
            println!("session_id: {session_id}");
            match snapshot.allowed_tools.as_ref() {
                Some(tools) => println!("allowed_tools: {}", tools.join(", ")),
                None => println!("allowed_tools: (none)"),
            }
            match snapshot.allowed_skills.as_ref() {
                Some(skills) => println!("allowed_skills: {}", skills.join(", ")),
                None => println!("allowed_skills: (none)"),
            }
            println!(
                "skill_policy: allowlist={} require_approval={} auto_approve_readonly={}",
                snapshot.skill_policy.allowlist.len(),
                snapshot.skill_policy.require_approval,
                snapshot.skill_policy.auto_approve_readonly
            );
            if !snapshot.skill_policy.allowlist.is_empty() {
                println!(
                    "skill_policy_allowlist: {}",
                    snapshot.skill_policy.allowlist.join(", ")
                );
            }
            println!();
            if snapshot.mcp_config_json.is_none() {
                println!("mcp: (none)");
                return Ok(());
            }
            println!("mcp: present");
            if let Some(servers) = mcp_servers {
                if servers.is_empty() {
                    println!("mcp_servers: (none)");
                } else {
                    println!("mcp_servers: {}", servers.join(", "));
                }
            }
            if let Some(err) = mcp_parse_error {
                println!("mcp_config_parse_error: {err}");
            }
            if let Some(cfg) = mcp_config_sanitized {
                println!();
                println!("mcp_config (sanitized):");
                println!("{}", serde_json::to_string_pretty(&cfg)?);
            }
            Ok(())
        }
    }
}

fn summarize_mcp_config(
    raw_json: Option<&str>,
    include_sanitized: bool,
) -> (Option<Vec<String>>, Option<Value>, Option<String>) {
    let Some(raw_json) = raw_json else {
        return (None, None, None);
    };

    let parsed: Value = match serde_json::from_str(raw_json) {
        Ok(v) => v,
        Err(err) => {
            return (None, None, Some(format!("invalid JSON: {err}")));
        }
    };

    let servers = parsed
        .get("servers")
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().cloned().collect::<Vec<_>>());

    let sanitized = if include_sanitized {
        Some(sanitize_mcp_config(&parsed))
    } else {
        None
    };

    (servers, sanitized, None)
}

fn sanitize_mcp_config(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if k == "env" {
                    match v.as_object() {
                        Some(env) => {
                            let mut redacted = serde_json::Map::new();
                            for (ek, _ev) in env {
                                redacted
                                    .insert(ek.clone(), Value::String("[redacted]".to_string()));
                            }
                            out.insert(k.clone(), Value::Object(redacted));
                        }
                        None => {
                            out.insert(k.clone(), Value::String("[redacted]".to_string()));
                        }
                    }
                    continue;
                }
                out.insert(k.clone(), sanitize_mcp_config(v));
            }
            Value::Object(out)
        }
        Value::Array(values) => Value::Array(values.iter().map(sanitize_mcp_config).collect()),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_mcp_config_redacts_env_values() {
        let input = serde_json::json!({
            "servers": {
                "s1": {
                    "command": "python",
                    "env": {
                        "API_KEY": "secret",
                        "TOKEN": "abc",
                    }
                }
            }
        });

        let sanitized = sanitize_mcp_config(&input);
        assert_eq!(
            sanitized["servers"]["s1"]["env"]["API_KEY"].as_str(),
            Some("[redacted]")
        );
        assert_eq!(
            sanitized["servers"]["s1"]["env"]["TOKEN"].as_str(),
            Some("[redacted]")
        );
        assert_eq!(
            sanitized["servers"]["s1"]["command"].as_str(),
            Some("python")
        );
    }
}

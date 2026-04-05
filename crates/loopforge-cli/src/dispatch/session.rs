use std::collections::BTreeMap;

use anyhow::Context;
use serde_json::Value;

use super::mcp_sanitize::sanitize_mcp_config;
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

            if json {
                let out = build_session_policy_json(
                    session_id,
                    &snapshot.allowed_tools,
                    &snapshot.allowed_skills,
                    &snapshot.skill_policy,
                    snapshot.mcp_config_json.as_deref(),
                    show_mcp_config,
                )?;
                println!("{}", serde_json::to_string_pretty(&out)?);
                return Ok(());
            }

            let (mcp_servers, mcp_config_sanitized, mcp_parse_error) =
                summarize_mcp_config(snapshot.mcp_config_json.as_deref(), show_mcp_config);

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

fn build_session_policy_json(
    session_id: String,
    allowed_tools: &Option<Vec<String>>,
    allowed_skills: &Option<Vec<String>>,
    skill_policy: &rexos::agent::SessionSkillPolicy,
    mcp_config_json: Option<&str>,
    show_mcp_config: bool,
) -> anyhow::Result<Value> {
    let (mcp_servers, mcp_config_sanitized, mcp_parse_error) =
        summarize_mcp_config(mcp_config_json, show_mcp_config);

    let mut out = BTreeMap::new();
    out.insert("session_id".to_string(), Value::String(session_id));
    out.insert(
        "allowed_tools".to_string(),
        serde_json::to_value(allowed_tools)?,
    );
    out.insert(
        "allowed_skills".to_string(),
        serde_json::to_value(allowed_skills)?,
    );
    out.insert(
        "skill_policy".to_string(),
        serde_json::to_value(skill_policy)?,
    );

    let mut mcp = BTreeMap::new();
    mcp.insert(
        "present".to_string(),
        Value::Bool(mcp_config_json.is_some()),
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

    Ok(Value::Object(out.into_iter().collect()))
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
        .map(|obj| {
            let mut servers = obj.keys().cloned().collect::<Vec<_>>();
            servers.sort();
            servers
        });

    let sanitized = if include_sanitized {
        Some(sanitize_mcp_config(&parsed))
    } else {
        None
    };

    (servers, sanitized, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_session_policy_json_includes_expected_keys() {
        let mut skill_policy = rexos::agent::SessionSkillPolicy::default();
        skill_policy.allowlist = vec!["alpha".to_string()];
        skill_policy.require_approval = true;
        skill_policy.auto_approve_readonly = false;

        let mcp_config_json =
            r#"{"servers":{"s1":{"command":"python","env":{"API_KEY":"secret"}}}}"#;
        let out = build_session_policy_json(
            "s-test".to_string(),
            &Some(vec!["fs_read".to_string(), "fs_write".to_string()]),
            &None,
            &skill_policy,
            Some(mcp_config_json),
            true,
        )
        .unwrap();

        assert_eq!(out["session_id"].as_str(), Some("s-test"));
        assert!(out.get("allowed_tools").is_some());
        assert!(out.get("allowed_skills").is_some());
        assert!(out.get("skill_policy").is_some());
        assert!(out.get("mcp").is_some());

        let tools: Vec<String> = out["allowed_tools"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        assert!(tools.contains(&"fs_read".to_string()));
        assert!(tools.contains(&"fs_write".to_string()));

        assert_eq!(
            out["skill_policy"]["require_approval"].as_bool(),
            Some(true)
        );
        assert_eq!(
            out["skill_policy"]["auto_approve_readonly"].as_bool(),
            Some(false)
        );
        assert_eq!(out["skill_policy"]["allowlist"][0].as_str(), Some("alpha"));

        assert_eq!(out["mcp"]["present"].as_bool(), Some(true));
        let servers: Vec<String> = out["mcp"]["servers"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        assert_eq!(servers, vec!["s1".to_string()]);
        assert_eq!(
            out["mcp"]["config"]["servers"]["s1"]["env"]["API_KEY"].as_str(),
            Some("[redacted]")
        );
        assert_eq!(
            out["mcp"]["config"]["servers"]["s1"]["command"].as_str(),
            Some("python")
        );
    }

    #[test]
    fn build_session_policy_json_omits_sanitized_config_when_not_requested() {
        let out = build_session_policy_json(
            "s-test".to_string(),
            &None,
            &None,
            &rexos::agent::SessionSkillPolicy::default(),
            Some(r#"{"servers":{"s1":{"command":"python"}}}"#),
            false,
        )
        .unwrap();

        let mcp = out["mcp"].as_object().unwrap();
        assert_eq!(mcp.get("present").and_then(|v| v.as_bool()), Some(true));
        assert!(mcp.contains_key("servers"));
        assert!(!mcp.contains_key("config"));
    }

    #[test]
    fn build_session_policy_json_reports_invalid_mcp_json() {
        let out = build_session_policy_json(
            "s-test".to_string(),
            &None,
            &None,
            &rexos::agent::SessionSkillPolicy::default(),
            Some("not-json"),
            true,
        )
        .unwrap();

        let mcp = out["mcp"].as_object().unwrap();
        assert_eq!(mcp.get("present").and_then(|v| v.as_bool()), Some(true));
        assert!(mcp.get("servers").is_none());
        assert!(mcp.get("config").is_none());
        assert!(mcp
            .get("parse_error")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .starts_with("invalid JSON:"),);
    }

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

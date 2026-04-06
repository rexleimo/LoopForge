use super::super::mcp_sanitize::sanitize_mcp_config;
use super::json::build_session_policy_json;

#[test]
fn build_session_policy_json_includes_expected_keys() {
    let skill_policy = rexos::agent::SessionSkillPolicy {
        allowlist: vec!["alpha".to_string()],
        require_approval: true,
        auto_approve_readonly: false,
    };

    let mcp_config_json = r#"{"servers":{"s1":{"command":"python","env":{"API_KEY":"secret"}}}}"#;
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

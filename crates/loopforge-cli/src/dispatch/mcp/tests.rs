use super::super::mcp_sanitize::sanitize_mcp_config;
use super::json::{build_mcp_diagnose_json, normalize_json_string};

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

use std::collections::BTreeMap;

use serde_json::Value;

use super::mcp::summarize_mcp_config;

pub(super) fn build_session_policy_json(
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

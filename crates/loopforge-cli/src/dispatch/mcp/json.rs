use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Context;
use serde_json::Value;

pub(super) fn build_mcp_diagnose_json(
    workspace: &Path,
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

pub(super) fn normalize_json_string(raw: &str) -> anyhow::Result<String> {
    let json: Value = serde_json::from_str(raw).context("parse JSON")?;
    Ok(serde_json::to_string(&json)?)
}

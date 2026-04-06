use serde_json::Value;

use super::super::mcp_sanitize::sanitize_mcp_config;

pub(super) fn summarize_mcp_config(
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

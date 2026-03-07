use serde_json::Value;

pub(super) fn page_target_ws(targets: &[Value]) -> Option<String> {
    for target in targets {
        if target.get("type").and_then(|value| value.as_str()) == Some("page") {
            if let Some(ws) = target
                .get("webSocketDebuggerUrl")
                .and_then(|value| value.as_str())
            {
                return Some(ws.to_string());
            }
        }
    }
    None
}

use rexos_llm::openai_compat::{ToolCall, ToolFunction};

#[derive(Debug, serde::Deserialize)]
pub(super) struct JsonToolCall {
    #[serde(alias = "function_name")]
    name: String,
    #[serde(alias = "args")]
    #[serde(default)]
    arguments: Option<serde_json::Value>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

pub(super) fn into_tool_calls(calls: Vec<JsonToolCall>) -> Vec<ToolCall> {
    let mut out = Vec::new();
    for (idx, call) in calls.into_iter().enumerate() {
        let args_value = call
            .arguments
            .unwrap_or(serde_json::Value::Object(call.extra));
        let args = if let Some(text) = args_value.as_str() {
            text.to_string()
        } else {
            serde_json::to_string(&args_value).unwrap_or_else(|_| "{}".to_string())
        };
        out.push(ToolCall {
            id: format!("call_json_{}", idx + 1),
            kind: "function".to_string(),
            function: ToolFunction {
                name: call.name,
                arguments: args,
            },
        });
    }
    out
}

pub(super) fn parse_json_tool_calls_from_value(
    value: serde_json::Value,
) -> Option<Vec<JsonToolCall>> {
    if let Some(array) = value.as_array() {
        let mut calls = Vec::new();
        for item in array {
            calls.push(serde_json::from_value::<JsonToolCall>(item.clone()).ok()?);
        }
        return Some(calls);
    }

    serde_json::from_value::<JsonToolCall>(value)
        .ok()
        .map(|call| vec![call])
}

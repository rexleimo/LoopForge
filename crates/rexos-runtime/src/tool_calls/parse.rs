mod json;
mod scan;

use rexos_llm::openai_compat::ToolCall;

use json::{into_tool_calls, parse_json_tool_calls_from_value};
use scan::extract_json_tool_calls_from_text;

pub(crate) fn normalize_tool_arguments(tool_name: &str, raw_arguments_json: &str) -> String {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(raw_arguments_json) else {
        return raw_arguments_json.to_string();
    };

    let Some(object) = value.as_object() else {
        return raw_arguments_json.to_string();
    };

    let matches_name = object
        .get("function")
        .and_then(|value| value.as_str())
        .or_else(|| object.get("name").and_then(|value| value.as_str()))
        .or_else(|| object.get("function_name").and_then(|value| value.as_str()))
        .map(|name| name == tool_name)
        .unwrap_or(true);
    if !matches_name {
        return raw_arguments_json.to_string();
    }

    let Some(inner) = object.get("arguments") else {
        return raw_arguments_json.to_string();
    };

    if let Some(text) = inner.as_str() {
        return text.to_string();
    }

    serde_json::to_string(inner).unwrap_or_else(|_| raw_arguments_json.to_string())
}

pub(crate) fn parse_tool_calls_from_json_content(content: &str) -> Option<Vec<ToolCall>> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) {
        if let Some(calls) = parse_json_tool_calls_from_value(value) {
            return Some(into_tool_calls(calls));
        }
    }

    let calls = extract_json_tool_calls_from_text(trimmed);
    if calls.is_empty() {
        return None;
    }
    Some(into_tool_calls(calls))
}

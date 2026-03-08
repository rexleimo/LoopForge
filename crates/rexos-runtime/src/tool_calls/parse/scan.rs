use super::json::{parse_json_tool_calls_from_value, JsonToolCall};

pub(super) fn extract_json_tool_calls_from_text(content: &str) -> Vec<JsonToolCall> {
    let mut calls = Vec::new();
    for (start, _) in content.match_indices('{') {
        if calls.len() >= 16 {
            break;
        }
        let Some(end) = find_balanced_json_object_end(content, start) else {
            continue;
        };
        let slice = &content[start..end];
        let Ok(value) = serde_json::from_str::<serde_json::Value>(slice) else {
            continue;
        };
        let Some(mut parsed) = parse_json_tool_calls_from_value(value) else {
            continue;
        };
        calls.append(&mut parsed);
    }
    calls
}

fn find_balanced_json_object_end(content: &str, start: usize) -> Option<usize> {
    let bytes = content.as_bytes();
    if start >= bytes.len() || bytes[start] != b'{' {
        return None;
    }

    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escape = false;

    for (index, &byte) in bytes.iter().enumerate().skip(start) {
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            if byte == b'\\' {
                escape = true;
                continue;
            }
            if byte == b'"' {
                in_string = false;
                continue;
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
    }

    None
}

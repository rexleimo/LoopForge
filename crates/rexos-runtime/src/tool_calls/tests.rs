use super::{
    normalize_tool_arguments, parse_tool_calls_from_json_content, truncate_tool_result_with_flag,
};

#[test]
fn parses_embedded_json_tool_calls_from_freeform_text() {
    let content = r#"Before text {"function_name":"fs_write","args":{"path":"hello.txt","content":"hi"}} after"#;
    let calls = parse_tool_calls_from_json_content(content).expect("expected parsed tool call");
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].function.name, "fs_write");
}

#[test]
fn unwraps_nested_tool_arguments_payload() {
    let raw = r#"{"function_name":"fs_write","arguments":{"path":"a.txt"}}"#;
    assert_eq!(
        normalize_tool_arguments("fs_write", raw),
        r#"{"path":"a.txt"}"#
    );
}

#[test]
fn truncation_preserves_omission_marker() {
    let (out, truncated) = truncate_tool_result_with_flag("abcdefghij".to_string(), 6);
    assert!(truncated);
    assert!(out.contains("omitted"), "{out}");
}

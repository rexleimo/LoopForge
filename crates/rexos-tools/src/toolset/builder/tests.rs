use super::allowed::normalize_allowed_tools;

#[test]
fn normalize_allowed_tools_trims_drops_blank_and_dedupes() {
    let normalized = normalize_allowed_tools(vec![
        "  read_file  ".to_string(),
        String::new(),
        "read_file".to_string(),
        "  browser_open".to_string(),
        "   ".to_string(),
    ]);

    assert_eq!(normalized.len(), 2);
    assert!(normalized.contains("read_file"));
    assert!(normalized.contains("browser_open"));
}

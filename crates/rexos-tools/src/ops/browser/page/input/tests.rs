use super::validate_browser_expression;

#[test]
fn validate_browser_expression_rejects_empty_and_large_scripts() {
    assert!(validate_browser_expression("   ").is_err());
    assert!(validate_browser_expression(&"x".repeat(100_001)).is_err());
    assert!(validate_browser_expression("1 + 1").is_ok());
}

use super::location::non_empty_env_value;

#[test]
fn non_empty_env_value_filters_blank_strings() {
    assert_eq!(
        non_empty_env_value(Some("Asia/Shanghai")),
        Some("Asia/Shanghai".to_string())
    );
    assert_eq!(non_empty_env_value(Some("   ")), None);
    assert_eq!(non_empty_env_value(None), None);
}

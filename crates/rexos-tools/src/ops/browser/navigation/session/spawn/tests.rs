use super::resolved_headless;

#[test]
fn resolved_headless_prefers_explicit_value() {
    assert!(resolved_headless(Some(true), false));
    assert!(!resolved_headless(Some(false), true));
    assert!(resolved_headless(None, true));
}

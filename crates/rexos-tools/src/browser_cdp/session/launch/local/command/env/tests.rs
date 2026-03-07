use super::allow::allowed_chromium_env_keys;

#[test]
fn allowed_chromium_env_keys_keep_minimum_runtime_variables() {
    let keys = allowed_chromium_env_keys();
    assert!(keys.contains(&"PATH"));
    assert!(keys.contains(&"HOME") || keys.contains(&"USERPROFILE"));
    assert!(keys.contains(&"TMP") || keys.contains(&"TMPDIR"));
}

use super::names::browser_binary_names;

#[test]
fn browser_binary_names_include_common_chrome_variants() {
    let names = browser_binary_names();
    assert!(names.contains(&"google-chrome"));
    assert!(names.contains(&"chromium"));
    assert!(names.contains(&"msedge"));
}

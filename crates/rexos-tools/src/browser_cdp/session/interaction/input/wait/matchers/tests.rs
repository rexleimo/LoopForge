use super::{selector_check_script, text_check_script};

#[test]
fn selector_check_script_uses_query_selector() {
    let script = selector_check_script("#submit");
    assert!(script.contains("document.querySelector"));
    assert!(script.contains("#submit"));
}

#[test]
fn text_check_script_uses_inner_text_match() {
    let script = text_check_script("hello");
    assert!(script.contains("document.body.innerText.includes"));
    assert!(script.contains("hello"));
}

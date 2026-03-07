pub(super) fn selector_check_script(selector: &str) -> String {
    let selector_json = serde_json::to_string(selector).unwrap_or_default();
    format!("document.querySelector({selector_json}) ? 'found' : null")
}

pub(super) fn text_check_script(text: &str) -> String {
    let text_json = serde_json::to_string(text).unwrap_or_default();
    format!(
        "document.body && document.body.innerText && document.body.innerText.includes({text_json}) ? 'found' : null"
    )
}

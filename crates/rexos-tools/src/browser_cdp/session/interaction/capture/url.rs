use serde_json::Value;

pub(super) fn page_url_from_value(value: Option<Value>) -> String {
    value
        .and_then(|value| value.as_str().map(String::from))
        .unwrap_or_default()
}

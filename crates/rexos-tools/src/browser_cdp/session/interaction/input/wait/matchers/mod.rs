mod check;
mod script;
#[cfg(test)]
mod tests;

use serde_json::{Map, Value};

use crate::browser_cdp::session::CdpBrowserSession;

fn selector_check_script(selector: &str) -> String {
    script::selector_check_script(selector)
}

fn text_check_script(text: &str) -> String {
    script::text_check_script(text)
}

pub(super) async fn waited_for_matches(
    session: &CdpBrowserSession,
    selector: Option<&str>,
    text: Option<&str>,
) -> Map<String, Value> {
    let mut waited_for = Map::new();

    if let Some(selector) = selector {
        if check::selector_found(session, &selector_check_script(selector)).await {
            waited_for.insert("selector".to_string(), Value::String(selector.to_string()));
        }
    }

    if let Some(text) = text {
        if check::text_found(session, &text_check_script(text)).await {
            waited_for.insert("text".to_string(), Value::String(text.to_string()));
        }
    }

    waited_for
}

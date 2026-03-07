use anyhow::Context;
use serde_json::Value;

use super::helpers::{ensure_action_success, parse_action_result};
use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn type_text(&self, selector: &str, text: &str) -> anyhow::Result<Value> {
        let selector_json = serde_json::to_string(selector).unwrap_or_default();
        let text_json = serde_json::to_string(text).unwrap_or_default();
        let js = format!(
            r#"(() => {{
    let sel = {selector_json};
    let txt = {text_json};
    let el = document.querySelector(sel);
    if (!el) return JSON.stringify({{success: false, error: 'Input not found: ' + sel}});
    el.focus();
    el.value = txt;
    el.dispatchEvent(new Event('input', {{bubbles: true}}));
    el.dispatchEvent(new Event('change', {{bubbles: true}}));
    return JSON.stringify({{success: true, selector: sel, typed: txt.length + ' chars'}});
}})()"#
        );

        let value = self.cdp.run_js(&js).await.context("type js")?;
        let parsed = parse_action_result(value);
        ensure_action_success(&parsed, "type failed")?;
        Ok(parsed)
    }
}

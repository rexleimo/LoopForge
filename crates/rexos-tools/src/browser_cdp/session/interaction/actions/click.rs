use std::time::Duration;

use anyhow::Context;
use serde_json::Value;

use super::helpers::{ensure_action_success, parse_action_result};
use crate::browser_cdp::session::helpers::{page_info, wait_for_load};
use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn click(&self, selector: &str) -> anyhow::Result<Value> {
        let sel_json = serde_json::to_string(selector).unwrap_or_default();
        let js = format!(
            r#"(() => {{
    let sel = {sel_json};
    let el = document.querySelector(sel);
    if (!el) {{
        const all = document.querySelectorAll('a, button, [role="button"], input[type="submit"], [onclick]');
        const lower = sel.toLowerCase();
        for (const e of all) {{
            if (e.textContent.trim().toLowerCase().includes(lower)) {{ el = e; break; }}
        }}
    }}
    if (!el) return JSON.stringify({{success: false, error: 'Element not found: ' + sel}});
    el.scrollIntoView({{block: 'center'}});
    el.click();
    return JSON.stringify({{success: true, tag: el.tagName, text: el.textContent.substring(0, 100).trim()}});
}})()"#
        );

        let value = self.cdp.run_js(&js).await.context("click js")?;
        let parsed = parse_action_result(value);
        ensure_action_success(&parsed, "click failed")?;

        tokio::time::sleep(Duration::from_millis(500)).await;
        wait_for_load(&self.cdp).await;
        page_info(&self.cdp).await.or(Ok(parsed))
    }
}

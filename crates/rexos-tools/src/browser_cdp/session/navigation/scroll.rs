use anyhow::Context;
use serde_json::Value;

use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn scroll(&self, direction: &str, amount: i64) -> anyhow::Result<Value> {
        let (dx, dy) = scroll_delta(direction, amount);
        let js = format!(
            r#"(() => {{
  window.scrollBy({dx}, {dy});
  return {{
    scrollX: (typeof window.scrollX === 'number' ? window.scrollX : (window.pageXOffset || 0)),
    scrollY: (typeof window.scrollY === 'number' ? window.scrollY : (window.pageYOffset || 0)),
  }};
}})()"#
        );

        self.cdp.run_js(&js).await.context("scroll js")
    }
}

fn scroll_delta(direction: &str, amount: i64) -> (i64, i64) {
    match direction {
        "up" => (0, -amount),
        "down" => (0, amount),
        "left" => (-amount, 0),
        "right" => (amount, 0),
        _ => (0, amount),
    }
}

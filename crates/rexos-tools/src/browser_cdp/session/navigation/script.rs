use anyhow::Context;
use serde_json::Value;

use crate::browser_cdp::session::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn run_js(&self, expression: &str) -> anyhow::Result<Value> {
        self.cdp.run_js(expression).await.context("run js")
    }

    pub async fn current_url(&self) -> anyhow::Result<String> {
        let value = self
            .cdp
            .run_js("location.href")
            .await
            .context("location.href")?;
        Ok(value.as_str().unwrap_or("").to_string())
    }
}

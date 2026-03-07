#[path = "capture/payload.rs"]
mod payload;
#[path = "capture/read.rs"]
mod read;
#[path = "capture/request.rs"]
mod request;
#[cfg(test)]
#[path = "capture/tests.rs"]
mod tests;
#[path = "capture/url.rs"]
mod url;

use anyhow::Context;
use serde_json::Value;

use super::super::helpers::EXTRACT_CONTENT_JS;
use super::super::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn read_page(&self) -> anyhow::Result<Value> {
        let val = self
            .cdp
            .run_js(EXTRACT_CONTENT_JS)
            .await
            .context("read page js")?;
        Ok(read::parse_read_page_value(val))
    }

    pub async fn screenshot(&self) -> anyhow::Result<Value> {
        let result = self
            .cdp
            .send(
                "Page.captureScreenshot",
                request::capture_screenshot_params(),
            )
            .await
            .context("Page.captureScreenshot")?;
        let b64 = result.get("data").and_then(|v| v.as_str()).unwrap_or("");
        let url = self.cdp.run_js("location.href").await.ok();
        Ok(payload::screenshot_result(
            url::page_url_from_value(url),
            b64,
        ))
    }
}

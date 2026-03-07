mod body;
mod request;

use std::time::Duration;

use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn web_fetch(
        &self,
        url: &str,
        timeout_ms: Option<u64>,
        max_bytes: Option<u64>,
        allow_private: bool,
    ) -> anyhow::Result<String> {
        let url = reqwest::Url::parse(url).context("parse url")?;
        super::ensure_remote_url_allowed(&url, allow_private).await?;

        let timeout = Duration::from_millis(timeout_ms.unwrap_or(20_000));
        let max_bytes = max_bytes.unwrap_or(200_000) as usize;
        let response = request::send_request(&self.http, url, timeout).await?;
        let status = response.status().as_u16();
        let content_type = request::content_type(&response);
        let bytes = request::read_body(response, timeout).await?;
        let (body, truncated, bytes_returned) = body::format_fetch_body(&bytes, max_bytes);

        Ok(serde_json::json!({
            "status": status,
            "content_type": content_type,
            "body": body,
            "truncated": truncated,
            "bytes": bytes_returned,
            "total_bytes": bytes.len(),
        })
        .to_string())
    }
}

use std::time::Duration;

use anyhow::Context;

pub(super) async fn send_request(
    http: &reqwest::Client,
    url: reqwest::Url,
    timeout: Duration,
) -> anyhow::Result<reqwest::Response> {
    tokio::time::timeout(timeout, http.get(url).send())
        .await
        .context("web_fetch timed out")?
        .context("send request")
}

pub(super) fn content_type(response: &reqwest::Response) -> String {
    response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string()
}

pub(super) async fn read_body(
    response: reqwest::Response,
    timeout: Duration,
) -> anyhow::Result<Vec<u8>> {
    let bytes = tokio::time::timeout(timeout, response.bytes())
        .await
        .context("web_fetch timed out")?
        .context("read response body")?;
    Ok(bytes.to_vec())
}

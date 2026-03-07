use anyhow::{bail, Context};

use super::super::A2A_USER_AGENT;

pub(super) async fn send_request(
    client: &reqwest::Client,
    url: reqwest::Url,
    body: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let resp = client
        .post(url)
        .header("User-Agent", A2A_USER_AGENT)
        .json(&body)
        .send()
        .await
        .context("send a2a_send request")?;

    ensure_success_status(resp.status())?;
    resp.json().await.context("parse a2a_send response")
}

pub(super) fn ensure_success_status(status: reqwest::StatusCode) -> anyhow::Result<()> {
    if status.is_success() {
        return Ok(());
    }
    bail!("a2a_send http {status}")
}

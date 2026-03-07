use anyhow::{bail, Context};

use crate::Toolset;

use super::super::A2A_USER_AGENT;

impl Toolset {
    pub(crate) async fn a2a_discover(
        &self,
        url: &str,
        allow_private: bool,
    ) -> anyhow::Result<String> {
        let url = super::url::agent_card_url(url, allow_private).await?;
        let resp = self
            .http
            .get(url.clone())
            .header("User-Agent", A2A_USER_AGENT)
            .send()
            .await
            .context("send a2a_discover request")?;

        if !resp.status().is_success() {
            bail!("a2a_discover http {}", resp.status());
        }

        let bytes = resp
            .bytes()
            .await
            .context("read a2a_discover response body")?;
        if bytes.len() > 200_000 {
            bail!("agent card too large: {} bytes", bytes.len());
        }

        let value: serde_json::Value =
            serde_json::from_slice(&bytes).context("parse agent card json")?;
        Ok(serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()))
    }
}

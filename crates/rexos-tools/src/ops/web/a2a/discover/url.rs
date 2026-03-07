use anyhow::Context;

pub(super) async fn agent_card_url(url: &str, allow_private: bool) -> anyhow::Result<reqwest::Url> {
    let mut url = reqwest::Url::parse(url).context("parse url")?;
    super::super::super::ensure_remote_url_allowed(&url, allow_private).await?;
    url.set_path("/.well-known/agent.json");
    url.set_query(None);
    url.set_fragment(None);
    Ok(url)
}

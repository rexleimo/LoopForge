use anyhow::{bail, Context};

pub(super) fn ensure_non_empty_message(message: &str) -> anyhow::Result<()> {
    if message.trim().is_empty() {
        bail!("message is empty");
    }
    Ok(())
}

pub(super) fn parse_agent_url(agent_url: &str) -> anyhow::Result<reqwest::Url> {
    reqwest::Url::parse(agent_url).context("parse agent_url")
}

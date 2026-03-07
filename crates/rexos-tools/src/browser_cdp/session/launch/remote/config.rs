use anyhow::Context;

pub(super) fn configured_remote_base() -> anyhow::Result<Option<String>> {
    let Ok(value) = std::env::var("LOOPFORGE_BROWSER_CDP_HTTP") else {
        return Ok(None);
    };

    let value = value.trim().to_string();
    if value.is_empty() {
        return Ok(None);
    }

    parse_and_validate_remote_base(&value)?;
    Ok(Some(value))
}

pub(super) fn parse_and_validate_remote_base(base_http: &str) -> anyhow::Result<reqwest::Url> {
    let base = reqwest::Url::parse(base_http).context("parse LOOPFORGE_BROWSER_CDP_HTTP")?;
    crate::browser_cdp::discovery::validate_remote_cdp_base_url(&base)?;
    Ok(base)
}

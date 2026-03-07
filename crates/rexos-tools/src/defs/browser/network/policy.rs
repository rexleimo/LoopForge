use anyhow::{bail, Context};

pub(super) fn allowed_special_browser_url(url: &reqwest::Url) -> bool {
    matches!(
        (url.scheme(), url.as_str(), url.host_str()),
        ("about", "about:blank", _) | ("chrome-error", _, Some("chromewebdata"))
    )
}

pub(super) fn ensure_network_scheme(url: &reqwest::Url) -> anyhow::Result<()> {
    match url.scheme() {
        "http" | "https" => Ok(()),
        _ => bail!("only http/https urls are allowed"),
    }
}

pub(super) fn browser_url_host_port(url: &reqwest::Url) -> anyhow::Result<(String, u16)> {
    let host = url.host_str().context("url missing host")?;
    let port = url.port_or_known_default().context("url missing port")?;
    Ok((host.to_string(), port))
}

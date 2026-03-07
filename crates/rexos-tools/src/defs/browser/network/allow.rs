use anyhow::{bail, Context};

use crate::net::is_forbidden_ip;

pub(crate) async fn ensure_browser_url_allowed(
    url: &str,
    allow_private: bool,
) -> anyhow::Result<()> {
    let url = reqwest::Url::parse(url).context("parse url")?;

    if super::allowed_special_browser_url(&url) {
        return Ok(());
    }

    super::policy::ensure_network_scheme(&url)?;

    if allow_private {
        return Ok(());
    }

    let (host, port) = super::browser_url_host_port(&url)?;
    let ips = super::resolve_host_ips(&host, port)
        .await
        .with_context(|| format!("resolve {host}:{port}"))?;

    for ip in ips {
        if is_forbidden_ip(ip) {
            bail!("url resolves to loopback/private address: {ip}");
        }
    }

    Ok(())
}

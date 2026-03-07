use anyhow::{bail, Context};

use crate::defs::resolve_host_ips;
use crate::net::is_forbidden_ip;

pub(super) async fn validated_browser_url(
    url: &str,
    allow_private: bool,
) -> anyhow::Result<reqwest::Url> {
    let url = reqwest::Url::parse(url).context("parse url")?;
    match url.scheme() {
        "http" | "https" => {}
        _ => bail!("only http/https urls are allowed"),
    }

    let host = url.host_str().context("url missing host")?;
    let port = url.port_or_known_default().context("url missing port")?;

    if !allow_private {
        let ips = resolve_host_ips(host, port)
            .await
            .with_context(|| format!("resolve {host}:{port}"))?;
        for ip in ips {
            if is_forbidden_ip(ip) {
                bail!("url resolves to loopback/private address: {ip}");
            }
        }
    }

    Ok(url)
}

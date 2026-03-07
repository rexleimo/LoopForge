use anyhow::{bail, Context};

use crate::defs::resolve_host_ips;
use crate::net::is_forbidden_ip;

fn remote_url_host_port(url: &reqwest::Url) -> anyhow::Result<(String, u16)> {
    let host = url.host_str().context("url missing host")?;
    let port = url.port_or_known_default().context("url missing port")?;
    Ok((host.to_string(), port))
}

fn ensure_remote_scheme(url: &reqwest::Url) -> anyhow::Result<()> {
    match url.scheme() {
        "http" | "https" => Ok(()),
        _ => bail!("only http/https urls are allowed"),
    }
}

pub(super) async fn ensure_remote_url_allowed(
    url: &reqwest::Url,
    allow_private: bool,
) -> anyhow::Result<()> {
    ensure_remote_scheme(url)?;

    if !allow_private {
        let (host, port) = remote_url_host_port(url)?;
        let ips = resolve_host_ips(&host, port)
            .await
            .with_context(|| format!("resolve {host}:{port}"))?;
        for ip in ips {
            if is_forbidden_ip(ip) {
                bail!("url resolves to loopback/private address: {ip}");
            }
        }
    }

    Ok(())
}

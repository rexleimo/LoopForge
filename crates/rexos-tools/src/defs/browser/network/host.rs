use std::net::IpAddr;

use anyhow::{bail, Context};

pub(super) async fn resolve_host_ips(host: &str, port: u16) -> anyhow::Result<Vec<IpAddr>> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(vec![ip]);
    }

    let addrs = tokio::net::lookup_host((host, port))
        .await
        .context("dns lookup")?;

    let mut ips = Vec::new();
    for socket_addr in addrs {
        ips.push(socket_addr.ip());
    }

    if ips.is_empty() {
        bail!("no addresses found");
    }

    ips.sort();
    ips.dedup();
    Ok(ips)
}

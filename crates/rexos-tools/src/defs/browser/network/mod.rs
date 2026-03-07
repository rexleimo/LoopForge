mod allow;
mod host;
mod policy;
#[cfg(test)]
mod tests;

use std::net::IpAddr;

pub(crate) async fn resolve_host_ips(host: &str, port: u16) -> anyhow::Result<Vec<IpAddr>> {
    host::resolve_host_ips(host, port).await
}

fn allowed_special_browser_url(url: &reqwest::Url) -> bool {
    policy::allowed_special_browser_url(url)
}

fn browser_url_host_port(url: &reqwest::Url) -> anyhow::Result<(String, u16)> {
    policy::browser_url_host_port(url)
}

pub(crate) use allow::ensure_browser_url_allowed;

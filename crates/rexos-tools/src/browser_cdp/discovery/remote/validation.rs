use anyhow::{bail, Context};

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}

pub(crate) fn validate_remote_cdp_base_url(base: &reqwest::Url) -> anyhow::Result<()> {
    match base.scheme() {
        "http" | "https" => {}
        _ => bail!("LOOPFORGE_BROWSER_CDP_HTTP must be http:// or https://"),
    }

    let host = base
        .host_str()
        .context("LOOPFORGE_BROWSER_CDP_HTTP missing host")?;
    if is_loopback_host(host) || allow_remote_cdp() {
        return Ok(());
    }

    anyhow::bail!(
        "LOOPFORGE_BROWSER_CDP_HTTP points to non-loopback host ({host}). This is powerful and unsafe by default. Set LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE=1 to allow (prefer a TLS tunnel / tailnet-only endpoint)."
    );
}

pub(super) fn is_loopback_base(base: &reqwest::Url) -> bool {
    base.host_str().map(is_loopback_host).unwrap_or(false)
}

fn allow_remote_cdp() -> bool {
    std::env::var("LOOPFORGE_BROWSER_CDP_ALLOW_REMOTE")
        .ok()
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

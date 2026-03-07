mod a2a;
mod fetch;
mod location;
mod pdf;
mod remote;
mod search;
#[cfg(test)]
mod tests;

use crate::Toolset;

async fn ensure_remote_url_allowed(
    url: &reqwest::Url,
    allow_private: bool,
    tool_name: &str,
    method: &str,
    security: &rexos_kernel::security::SecurityConfig,
) -> anyhow::Result<()> {
    remote::ensure_remote_url_allowed(url, allow_private, tool_name, method, security).await
}

impl Toolset {
    pub(crate) fn location_get(&self) -> anyhow::Result<String> {
        location::location_get()
    }
}

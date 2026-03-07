mod a2a;
mod fetch;
mod location;
mod pdf;
mod remote;
mod search;
#[cfg(test)]
mod tests;

use crate::Toolset;

async fn ensure_remote_url_allowed(url: &reqwest::Url, allow_private: bool) -> anyhow::Result<()> {
    remote::ensure_remote_url_allowed(url, allow_private).await
}

impl Toolset {
    pub(crate) fn location_get(&self) -> anyhow::Result<String> {
        location::location_get()
    }
}

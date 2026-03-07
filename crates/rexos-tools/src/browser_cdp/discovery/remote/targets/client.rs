use std::time::Duration;

use anyhow::Context;

pub(super) fn loopback_http_client() -> anyhow::Result<reqwest::Client> {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .build()
        .context("build loopback CDP http client")
}

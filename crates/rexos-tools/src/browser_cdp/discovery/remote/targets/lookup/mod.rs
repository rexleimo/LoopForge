mod existing;
mod mode;
mod new_page;

use super::client::loopback_http_client;
use existing::find_existing_page_ws;
use new_page::try_new_page_ws;

pub(crate) async fn find_or_create_page_ws(
    http: &reqwest::Client,
    base: &reqwest::Url,
) -> anyhow::Result<String> {
    let loopback_client = if super::super::validation::is_loopback_base(base) {
        Some(loopback_http_client()?)
    } else {
        None
    };
    let http = loopback_client.as_ref().unwrap_or(http);

    if !mode::reuse_existing_tab() {
        if let Some(ws) = try_new_page_ws(http, base).await? {
            return Ok(ws);
        }
    }

    find_existing_page_ws(http, base).await
}

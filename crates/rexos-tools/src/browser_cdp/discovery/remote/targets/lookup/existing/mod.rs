mod page;
mod poll;
#[cfg(test)]
mod tests;

pub(super) async fn find_existing_page_ws(
    http: &reqwest::Client,
    base: &reqwest::Url,
) -> anyhow::Result<String> {
    poll::find_existing_page_ws(http, base).await
}

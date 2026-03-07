mod format;
mod parse;

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) async fn web_search(
        &self,
        query: &str,
        max_results: Option<u32>,
    ) -> anyhow::Result<String> {
        if query.trim().is_empty() {
            bail!("query is empty");
        }

        let max_results = max_results.unwrap_or(5).clamp(1, 20) as usize;
        let resp = self
            .http
            .get("https://html.duckduckgo.com/html/")
            .query(&[("q", query)])
            .header("User-Agent", "Mozilla/5.0 (compatible; LoopForge/0.1)")
            .send()
            .await
            .context("send web_search request")?
            .error_for_status()
            .context("web_search http error")?;

        let body = resp.text().await.context("read web_search body")?;
        let results = parse::parse_ddg_results(&body, max_results);
        Ok(format::format_search_results(query, results))
    }
}

mod decode;
mod html;

use self::decode::decode_result_url;
use self::html::{extract_snippet, extract_title};
use crate::net::extract_between;

pub(super) fn parse_ddg_results(html: &str, max: usize) -> Vec<(String, String, String)> {
    let mut results = Vec::new();

    for chunk in html.split("class=\"result__a\"") {
        if results.len() >= max {
            break;
        }
        if !chunk.contains("href=") {
            continue;
        }

        let url = extract_between(chunk, "href=\"", "\"")
            .unwrap_or_default()
            .to_string();
        let actual_url = decode_result_url(url);
        let title = extract_title(chunk);
        let snippet = extract_snippet(chunk);

        if !title.is_empty() && !actual_url.is_empty() {
            results.push((title, actual_url, snippet));
        }
    }

    results
}

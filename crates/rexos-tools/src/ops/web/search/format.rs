pub(super) fn format_search_results(query: &str, results: Vec<(String, String, String)>) -> String {
    if results.is_empty() {
        return format!("No results found for '{query}'.");
    }

    let mut out = format!("Search results for '{query}':\n\n");
    for (idx, (title, url, snippet)) in results.into_iter().enumerate() {
        out.push_str(&format!(
            "{}. {}\n   URL: {}\n   {}\n\n",
            idx + 1,
            title,
            url,
            snippet
        ));
    }
    out
}

use crate::net::extract_between;

pub(super) fn extract_title(chunk: &str) -> String {
    extract_between(chunk, ">", "</a>")
        .map(strip_html_tags)
        .unwrap_or_default()
}

pub(super) fn extract_snippet(chunk: &str) -> String {
    let Some(start) = chunk.find("class=\"result__snippet\"") else {
        return String::new();
    };

    let after = &chunk[start..];
    extract_between(after, ">", "</a>")
        .or_else(|| extract_between(after, ">", "</"))
        .map(strip_html_tags)
        .unwrap_or_default()
}

fn strip_html_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

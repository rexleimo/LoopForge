use anyhow::Context;

pub(super) fn devtools_url_from_line(line: &str) -> anyhow::Result<Option<String>> {
    if !line.contains("DevTools listening on") {
        return Ok(None);
    }

    let url = line
        .split("DevTools listening on ")
        .nth(1)
        .context("malformed DevTools URL line")?
        .trim()
        .to_string();
    Ok(Some(url))
}

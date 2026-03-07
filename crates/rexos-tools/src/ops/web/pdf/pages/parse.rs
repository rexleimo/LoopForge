use anyhow::{bail, Context};

pub(super) fn parse_range_bound(value: &str, context_label: &str) -> anyhow::Result<usize> {
    value
        .trim()
        .parse()
        .with_context(|| context_label.to_string())
}

pub(super) fn parse_page_number(part: &str) -> anyhow::Result<usize> {
    let number: usize = part.parse().context("parse page number")?;
    if number == 0 {
        bail!("pages are 1-indexed (got 0)");
    }
    Ok(number)
}

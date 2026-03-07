use anyhow::bail;

use super::parse::parse_range_bound;

pub(super) fn append_page_range(
    out: &mut Vec<usize>,
    original: &str,
    start: &str,
    end: &str,
) -> anyhow::Result<()> {
    let start = parse_range_bound(start, "parse pages range start")?;
    let end = parse_range_bound(end, "parse pages range end")?;
    if start == 0 || end == 0 {
        bail!("pages are 1-indexed (got {original})");
    }
    if end < start {
        bail!("pages range must be ascending (got {original})");
    }
    for number in start..=end {
        out.push(number);
    }
    Ok(())
}

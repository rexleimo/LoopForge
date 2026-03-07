mod boundary;
mod scan;
#[cfg(test)]
mod tests;

use anyhow::bail;

pub(super) fn validate_event_handler_attrs(lower: &str) -> anyhow::Result<()> {
    if scan::contains_event_handler_attr(lower) {
        bail!("forbidden event handler attribute detected (on* attributes are not allowed)");
    }
    Ok(())
}

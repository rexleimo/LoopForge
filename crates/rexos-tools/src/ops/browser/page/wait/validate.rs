use anyhow::bail;

pub(super) fn validate_browser_wait_selector(selector: &str) -> anyhow::Result<()> {
    if selector.trim().is_empty() {
        bail!("selector is empty");
    }
    Ok(())
}

pub(super) fn validate_browser_wait_for_inputs(
    selector: Option<&str>,
    text: Option<&str>,
) -> anyhow::Result<()> {
    if selector.unwrap_or("").trim().is_empty() && text.unwrap_or("").trim().is_empty() {
        bail!("browser_wait_for requires selector or text");
    }
    Ok(())
}

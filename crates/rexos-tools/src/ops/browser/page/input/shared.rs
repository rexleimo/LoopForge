use anyhow::bail;

pub(super) const BROWSER_SESSION_REQUIRED: &str =
    "browser session not started; call browser_navigate first";

pub(super) fn validate_browser_expression(expression: &str) -> anyhow::Result<()> {
    if expression.trim().is_empty() {
        bail!("expression is empty");
    }

    if expression.len() > 100_000 {
        bail!("expression too large");
    }

    Ok(())
}

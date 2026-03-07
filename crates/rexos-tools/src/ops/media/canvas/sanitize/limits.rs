use anyhow::bail;

pub(super) fn validate_html_size(html: &str, max_bytes: usize) -> anyhow::Result<()> {
    if html.trim().is_empty() {
        bail!("html is empty");
    }
    if html.len() > max_bytes {
        bail!("html too large: {} bytes (max {})", html.len(), max_bytes);
    }
    Ok(())
}

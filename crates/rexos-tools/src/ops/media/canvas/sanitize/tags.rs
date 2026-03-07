use anyhow::bail;

pub(super) fn validate_forbidden_tags(lower: &str) -> anyhow::Result<()> {
    for tag in [
        "<script", "</script", "<iframe", "</iframe", "<object", "</object", "<embed", "</embed",
        "<applet", "</applet",
    ] {
        if lower.contains(tag) {
            bail!("forbidden html tag detected: {tag}");
        }
    }
    Ok(())
}

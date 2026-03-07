use anyhow::bail;

pub(super) fn validate_forbidden_schemes(lower: &str) -> anyhow::Result<()> {
    for scheme in ["javascript:", "vbscript:", "data:text/html"] {
        if lower.contains(scheme) {
            bail!("forbidden url scheme detected: {scheme}");
        }
    }
    Ok(())
}

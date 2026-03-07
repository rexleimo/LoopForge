use anyhow::{bail, Context};

pub(super) fn patch_body<'a>(lines: &'a [&'a str]) -> anyhow::Result<&'a [&'a str]> {
    let begin = lines
        .iter()
        .position(|line| line.trim() == "*** Begin Patch")
        .context("missing '*** Begin Patch' marker")?;
    let end = lines
        .iter()
        .rposition(|line| line.trim() == "*** End Patch")
        .context("missing '*** End Patch' marker")?;
    if end <= begin {
        bail!("'*** End Patch' must come after '*** Begin Patch'");
    }
    Ok(&lines[begin + 1..end])
}

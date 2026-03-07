mod allow;
mod build;
#[cfg(test)]
mod tests;

pub(super) fn chromium_command(
    chrome_path: &std::path::Path,
    args: &[String],
) -> tokio::process::Command {
    build::chromium_command(chrome_path, args)
}

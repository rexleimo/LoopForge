mod args;
mod env;

pub(super) fn chromium_args(
    port: u16,
    user_data_dir: &std::path::Path,
    headless: bool,
) -> Vec<String> {
    args::chromium_args(port, user_data_dir, headless)
}

pub(super) fn chromium_command(
    chrome_path: &std::path::Path,
    args: &[String],
) -> tokio::process::Command {
    env::chromium_command(chrome_path, args)
}

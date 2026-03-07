mod base;
mod flags;
#[cfg(test)]
mod tests;

pub(super) fn chromium_args(
    port: u16,
    user_data_dir: &std::path::Path,
    headless: bool,
) -> Vec<String> {
    let mut args = base::base_chromium_args(port, user_data_dir);

    if headless {
        args = flags::with_headless_args(args);
    }

    if flags::no_sandbox_enabled() {
        flags::append_no_sandbox_args(&mut args);
    }

    args
}

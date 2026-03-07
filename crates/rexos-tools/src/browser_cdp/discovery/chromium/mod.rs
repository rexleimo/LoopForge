mod candidates;
mod config;
mod names;
mod path;
mod resolve;
#[cfg(test)]
mod tests;

pub(crate) fn find_chromium() -> anyhow::Result<std::path::PathBuf> {
    resolve::find_chromium()
}

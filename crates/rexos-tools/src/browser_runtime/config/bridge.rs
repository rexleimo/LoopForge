mod python;
mod script;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

pub(crate) fn browser_python_exe() -> String {
    python::browser_python_exe()
}

pub(crate) fn browser_bridge_script_path() -> anyhow::Result<PathBuf> {
    script::browser_bridge_script_path()
}

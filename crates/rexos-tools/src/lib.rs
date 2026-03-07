use std::path::PathBuf;
use std::sync::OnceLock;

pub(crate) const BROWSER_BRIDGE_SCRIPT: &str = include_str!("browser_bridge.py");
pub(crate) static BROWSER_BRIDGE_PATH: OnceLock<PathBuf> = OnceLock::new();

mod browser_cdp;
mod browser_runtime;
mod defs;
mod dispatch;
mod net;
mod ops;
mod patch;
mod process_runtime;
mod toolset;

pub use toolset::Toolset;

#[cfg(test)]
mod tests;

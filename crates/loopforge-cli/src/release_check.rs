mod git;
mod metadata;
mod public_docs;
mod render;
mod runner;
mod types;

pub(crate) use render::format_release_check_report;
pub(crate) use runner::run_release_check;
pub(crate) use types::{ReleaseCheckItem, ReleaseCheckReport};

#[cfg(test)]
mod tests;

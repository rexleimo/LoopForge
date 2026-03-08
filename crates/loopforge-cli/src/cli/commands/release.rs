use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub(crate) enum ReleaseCommand {
    /// Check release metadata and preflight conditions
    Check {
        /// Release tag, e.g. v0.1.0 (defaults to v<workspace version>)
        #[arg(long)]
        tag: Option<String>,
        /// Repository root to check (default: current directory)
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        /// Run `cargo test --workspace --locked` as part of preflight
        #[arg(long)]
        run_tests: bool,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

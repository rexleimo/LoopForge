#[derive(Debug, clap::Subcommand)]
pub(crate) enum ConfigCommand {
    /// Validate ~/.loopforge/config.toml and exit non-zero when invalid
    Validate {
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

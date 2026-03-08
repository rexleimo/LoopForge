#[derive(Debug, clap::Subcommand)]
pub(crate) enum AcpCommand {
    /// List recent ACP events
    Events {
        /// Optional session id filter
        #[arg(long)]
        session: Option<String>,
        /// Max events to print
        #[arg(long, default_value_t = 100)]
        limit: usize,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
    /// Show ACP delivery checkpoints for one session
    Checkpoints {
        /// Session id
        #[arg(long)]
        session: String,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

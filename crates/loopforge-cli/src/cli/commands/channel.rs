#[derive(Debug, clap::Subcommand)]
pub(crate) enum ChannelCommand {
    /// Drain queued outbox messages once
    Drain {
        /// Max messages to attempt in one run
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Run a long-lived worker that periodically drains the outbox
    Worker {
        /// Seconds between drain attempts
        #[arg(long, default_value_t = 5)]
        interval_secs: u64,
        /// Max messages to attempt per drain cycle
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
}

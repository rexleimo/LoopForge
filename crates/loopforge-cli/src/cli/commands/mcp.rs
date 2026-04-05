use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub(crate) enum McpCommand {
    /// Connect to MCP servers and report basic status (servers + tool list)
    Diagnose {
        /// Workspace directory (used for relative server cwd and for session resolution)
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Explicit session id (defaults to the workspace .loopforge/session_id)
        #[arg(long)]
        session: Option<String>,
        /// Path to an MCP servers JSON config (overrides per-session stored config)
        #[arg(long)]
        config: Option<PathBuf>,
        /// Also call `resources/list` on all servers
        #[arg(long)]
        resources: bool,
        /// Also call `prompts/list` on all servers
        #[arg(long)]
        prompts: bool,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

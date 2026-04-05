use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub(crate) enum SessionCommand {
    /// Show the session policy (allowed tools/skills, skill policy, MCP config presence)
    Policy {
        /// Workspace directory for session id resolution and policy reads
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Explicit session id (defaults to the workspace .loopforge/session_id)
        #[arg(long)]
        session: Option<String>,
        /// Include MCP config JSON (sanitized)
        #[arg(long)]
        show_mcp_config: bool,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub(crate) enum AgentCommand {
    /// Run a single agent session in a workspace
    Run {
        /// Workspace root directory (tools are sandboxed to this path)
        #[arg(long)]
        workspace: PathBuf,
        /// Optional MCP server config file (JSON; starts MCP servers and exposes their tools)
        #[arg(long)]
        mcp_config: Option<PathBuf>,
        /// User instruction for this run
        #[arg(long)]
        prompt: String,
        /// Optional system prompt (string)
        #[arg(long)]
        system: Option<String>,
        /// Optional session id (generated if omitted)
        #[arg(long)]
        session: Option<String>,
        /// Task kind for model routing
        #[arg(long, value_enum, default_value_t = AgentKind::Coding)]
        kind: AgentKind,
        /// Comma-separated allowed tool names for this session (session-level whitelist)
        #[arg(long, value_delimiter = ',')]
        allowed_tools: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum AgentKind {
    Planning,
    Coding,
    Summary,
}

impl From<AgentKind> for rexos::router::TaskKind {
    fn from(value: AgentKind) -> Self {
        match value {
            AgentKind::Planning => rexos::router::TaskKind::Planning,
            AgentKind::Coding => rexos::router::TaskKind::Coding,
            AgentKind::Summary => rexos::router::TaskKind::Summary,
        }
    }
}

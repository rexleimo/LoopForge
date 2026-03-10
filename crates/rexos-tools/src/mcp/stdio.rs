use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use super::jsonrpc::JsonRpcClient;
use super::McpServerConfig;

const STDERR_TAIL_LINES: usize = 80;

#[derive(Debug)]
pub(crate) struct StdioServer {
    pub(crate) client: JsonRpcClient,
    child: Arc<std::sync::Mutex<Child>>,
    stderr_tail: Arc<Mutex<Vec<String>>>,
}

impl StdioServer {
    pub(crate) fn stderr_tail(&self) -> Arc<Mutex<Vec<String>>> {
        self.stderr_tail.clone()
    }
}

impl Drop for StdioServer {
    fn drop(&mut self) {
        if let Ok(mut child) = self.child.lock() {
            let _ = child.start_kill();
        }
    }
}

pub(crate) async fn spawn_stdio_server(
    name: &str,
    cfg: &McpServerConfig,
    workspace_root: &Path,
) -> anyhow::Result<StdioServer> {
    if cfg.command.trim().is_empty() {
        return Err(anyhow!("mcp server '{name}' command is empty"));
    }

    let cwd = resolve_cwd(cfg, workspace_root)?;

    let mut cmd = Command::new(cfg.command.trim());
    cmd.args(cfg.args.iter().filter(|s| !s.trim().is_empty()));
    cmd.current_dir(&cwd);
    if !cfg.env.is_empty() {
        cmd.envs(cfg.env.iter());
    }
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("spawn mcp server '{name}'"))?;
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow!("missing stdin for mcp server '{name}'"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("missing stdout for mcp server '{name}'"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("missing stderr for mcp server '{name}'"))?;

    let stderr_tail: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let stderr_tail_writer = stderr_tail.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let mut guard = stderr_tail_writer.lock().await;
            guard.push(line);
            if guard.len() > STDERR_TAIL_LINES {
                let drain = guard.len() - STDERR_TAIL_LINES;
                guard.drain(0..drain);
            }
        }
    });

    let client = JsonRpcClient::new(stdout, stdin);
    Ok(StdioServer {
        client,
        child: Arc::new(std::sync::Mutex::new(child)),
        stderr_tail,
    })
}

fn resolve_cwd(cfg: &McpServerConfig, workspace_root: &Path) -> anyhow::Result<PathBuf> {
    match cfg.cwd.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        Some(value) => {
            let p = PathBuf::from(value);
            if p.is_relative() {
                Ok(workspace_root.join(p))
            } else {
                Ok(p)
            }
        }
        None => Ok(workspace_root.to_path_buf()),
    }
}

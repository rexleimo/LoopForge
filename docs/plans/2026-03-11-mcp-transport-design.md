# MCP Transport (stdio) Design

**Status:** approved  
**Date:** 2026-03-11

## Goal

Add **MCP (Model Context Protocol)** client support to LoopForge so an agent session can connect to one or more MCP servers and use:

- **Tools**: `tools/list` + `tools/call`
- **Resources**: `resources/list` + `resources/read`
- **Prompts**: `prompts/list` + `prompts/get`

This iteration targets **stdio transport** only, with a transport abstraction that can later add UDS/HTTP/OAuth without redesigning the tool interface.

## Non-goals (this iteration)

- Streaming model output improvements
- MCP server hosting mode (LoopForge acting as an MCP server)
- Non-stdio transports (UDS/HTTP/SSE) and OAuth flows
- Persisting MCP servers into `~/.loopforge/config.toml` presets

## User experience

### CLI (per-session)

`loopforge agent run` accepts an MCP servers config file:

```bash
loopforge agent run \
  --workspace my-ws \
  --mcp-config mcp-servers.json \
  --prompt "…"
```

This config is attached to the **session** (stored in memory KV under a session-scoped key), so the runtime can:

- start MCP servers at session start
- expose MCP tools to the model
- route tool calls to the right MCP server

### Naming and collision avoidance

Remote MCP tools are exposed as **namespaced** tools:

- `mcp_<server>__<tool>` (sanitized to `^[a-zA-Z0-9_-]{1,64}$`; internally mapped back to `{server, tool}`)

This prevents collisions with LoopForge’s built-in tools (`fs_*`, `shell_exec`, `web_fetch`, etc.).

Resources and prompts are exposed via wrapper tools (to avoid exploding tool counts and to keep schemas stable):

- `mcp_resources_list`, `mcp_resources_read`
- `mcp_prompts_list`, `mcp_prompts_get`
- (optional) `mcp_servers_list` for quick inspection

### Session allowlist interaction

If `--allowed-tools` is used, MCP tools follow the same filtering:

- users may allow a specific MCP tool name (e.g. `mcp_git__status`)
- wrapper tools can also be allowlisted independently

## Configuration format

The MCP config file is JSON (compatible with the docs-site MCP blog example):

```json
{
  "servers": {
    "filesystem": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-filesystem", "./"] },
    "git": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-git"] }
  }
}
```

Each server supports:

- `command` (required)
- `args` (optional)
- `env` (optional key-value map)
- `cwd` (optional working directory)

## Architecture

### Where it lives

- Tool definitions and execution are implemented in `rexos-tools` (Toolset).
- Session-scoped configuration is stored and read by `rexos-runtime` (AgentRuntime).
- CLI wiring is in `loopforge-cli`.

### Runtime flow

1. CLI loads `--mcp-config` JSON and stores it for the resolved `session_id`.
2. `AgentRuntime::run_session` loads MCP config (if present) and constructs a `Toolset` with MCP enabled.
3. On session start, Toolset initializes:
   - spawn stdio processes
   - MCP `initialize` / `initialized` handshake
   - `tools/list` to build tool definitions (cached for the session)
4. Agent loop runs normally; when the model calls:
   - `mcp_<server>__<tool>` → MCP `tools/call` on that server
   - `mcp_resources_*` / `mcp_prompts_*` → MCP resources/prompts methods

### Transport abstraction

Transport is separated into:

- `Transport` trait: send JSON-RPC message, read next message
- `StdioTransport`: spawns child process, uses newline-delimited JSON framing

This keeps future UDS/HTTP transports additive.

## Safety & guardrails

- MCP is **opt-in**: only enabled when `--mcp-config` is provided for a session.
- Tool calls still go through:
  - session tool whitelist (when set)
  - leak guard / truncation / audit logging (existing runtime pipeline)
- Tool name is always namespaced (`mcp_*`), avoiding shadowing built-in tool names.

## Error handling

- Invalid config JSON → session fails fast with a clear parse error.
- Server spawn failure / handshake failure / `tools/list` failure → session fails fast with context identifying the server.
- MCP method errors propagate as tool errors (captured by existing tool audit pipeline).

## Testing strategy

- Unit tests in `rexos-tools`:
  - `mcp` JSON-RPC framing and message routing
  - tool name mapping (`mcp_<server>__<tool>`)
  - wrapper tools schema and routing
- Integration-ish test using an in-tree MCP stdio stub:
  - supports `initialize`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `prompts/list`, `prompts/get`
  - validates that Toolset spawns, handshakes, and calls through correctly

## Docs (public)

- Update `docs-site/blog/mcp-integration-guide.md` to match the real CLI (`--mcp-config`) and remove placeholders for commands that do not exist yet.
- Add a short “MCP server config” reference page if needed after P1 stabilizes.

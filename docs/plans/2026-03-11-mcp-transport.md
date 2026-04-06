# MCP Transport (stdio) Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if available) or superpowers:executing-plans to implement this plan. Steps use markdown checkboxes for tracking.

**Goal:** Add MCP client support (stdio transport) so LoopForge can load an `mcp-servers.json`, expose MCP tools to the model, and route tool calls/resources/prompts through MCP.

**Architecture:** Session-scoped MCP config is stored in runtime KV. Toolset owns MCP transports and exposes namespaced tool definitions (`mcp_<server>__<tool>`) plus wrapper tools for resources/prompts.

**Tech Stack:** Rust, tokio, serde_json, newline-delimited JSON-RPC over stdio.

**Execution Status (2026-04-07):** Completed and merged on `main` (initial landing: `f33803e`; follow-up hardening and coverage included up to `0609812`).

**Re-Verification (2026-04-07):**
- `cargo test -p loopforge-cli --locked cli::tests -q` ✅
- `cargo test -p rexos-tools --locked mcp -q` ✅
- `cargo test -p rexos-tools --locked -q` ✅
- `cargo test -p rexos --locked agent_loop -q` ✅
- `python3 -m mkdocs build --strict` ✅
- `make check` ✅

---

## Chunk 1: CLI + session storage plumbing

### Task 1: Add `--mcp-config` flag and store on session

**Files:**
- Modify: `crates/loopforge-cli/src/cli/commands/agent.rs`
- Modify: `crates/loopforge-cli/src/dispatch/agent.rs`
- Modify: `crates/rexos-runtime/src/session_skills/storage.rs`
- Test: `crates/loopforge-cli/src/cli/tests.rs` (CLI parse)

- [x] **Step 1: Add `--mcp-config` to `loopforge agent run`**
  - Add `mcp_config: Option<PathBuf>` argument.

- [x] **Step 2: CLI reads JSON file and stores content for the session**
  - Read file bytes, parse JSON for validity, store normalized string in runtime session KV.

- [x] **Step 3: Add runtime APIs**
  - `set_session_mcp_config(session_id, raw_json)`
  - `load_session_mcp_config(session_id) -> Option<String>`

- [x] **Step 4: Update CLI tests**
  - Ensure the new flag parses.

- [x] **Step 5: Run focused tests**
  - Run: `cargo test -p loopforge-cli cli::tests -q`
  - Expected: PASS

---

## Chunk 2: Toolset MCP core (stdio + JSON-RPC)

### Task 2: Add MCP config structs and stdio transport

**Files:**
- Create: `crates/rexos-tools/src/mcp/mod.rs`
- Create: `crates/rexos-tools/src/mcp/config.rs`
- Create: `crates/rexos-tools/src/mcp/jsonrpc.rs`
- Create: `crates/rexos-tools/src/mcp/stdio.rs`
- Create: `crates/rexos-tools/src/mcp/types.rs`
- Test: `crates/rexos-tools/src/mcp/tests.rs`

- [x] **Step 1: Define config types**
  - `McpServersConfig { servers: BTreeMap<String, McpServerConfig> }`
  - `McpServerConfig { command, args, env, cwd }`

- [x] **Step 2: Implement JSON-RPC client helper**
  - `send_request(method, params) -> Result<Value>`
  - background read loop, oneshot per id, timeout support.

- [x] **Step 3: Implement stdio transport**
  - spawn child, newline-delimited JSON, capture stderr tail on failure.

- [x] **Step 4: Implement MCP handshake**
  - `initialize` then `initialized` notification.

- [x] **Step 5: Unit tests for JSON-RPC routing**
  - use a minimal in-process “fake transport” (or spawn stub binary) to validate request/response matching.

- [x] **Step 6: Run focused tests**
  - Run: `cargo test -p rexos-tools mcp -q`
  - Expected: PASS

---

## Chunk 3: Expose MCP tools/resources/prompts as LoopForge tools

### Task 3: Add MCP tool definitions + routing

**Files:**
- Modify: `crates/rexos-tools/src/lib.rs`
- Modify: `crates/rexos-tools/src/toolset/mod.rs`
- Modify: `crates/rexos-tools/src/toolset/defs.rs`
- Modify: `crates/rexos-tools/src/defs/catalog.rs`
- Create: `crates/rexos-tools/src/defs/mcp/mod.rs`
- Create: `crates/rexos-tools/src/defs/mcp/schema.rs`
- Modify: `crates/rexos-tools/src/dispatch/domain/mod.rs`
- Modify: `crates/rexos-tools/src/dispatch/domain/classify.rs`
- Create: `crates/rexos-tools/src/dispatch/domain/mcp.rs`
- Modify: `crates/rexos-tools/src/dispatch/routing.rs`
- Create: `crates/rexos-tools/src/dispatch/mcp/mod.rs`
- Test: `crates/rexos-tools/src/tests/compat.rs` (definitions include MCP wrappers when enabled)

- [x] **Step 1: Add MCP wrapper tool defs**
  - `mcp_resources_list`, `mcp_resources_read`, `mcp_prompts_list`, `mcp_prompts_get`, `mcp_servers_list`

- [x] **Step 2: Extend Toolset with optional MCP hub**
  - On build: parse raw JSON from runtime, initialize servers, list tools.

- [x] **Step 3: Flatten remote MCP tools into `ToolDefinition`s**
  - name: `mcp_<server>__<tool>`
  - parameters: remote `inputSchema` (default `{ "type": "object" }`)

- [x] **Step 4: Add dispatch routing**
  - New domain `ToolCallDomain::Mcp` and route calls to MCP hub.

- [x] **Step 5: Run focused tests**
  - Run: `cargo test -p rexos-tools -q`
  - Expected: PASS

---

## Chunk 4: Runtime integration (agent loop)

### Task 4: Use MCP-enabled tool definitions per session

**Files:**
- Modify: `crates/rexos-runtime/src/session_runner/chat_loop.rs`
- Modify: `crates/rexos-runtime/src/session_skills/storage.rs`
- Test: `crates/rexos/tests/agent_loop.rs` (add a small case with MCP stub)

- [x] **Step 1: Load session MCP config in `run_session`**
  - If present, build Toolset with MCP enabled.

- [x] **Step 2: Ensure tool defs include MCP tools**
  - Toolset initializes MCP before `tool_defs` is computed.

- [x] **Step 3: Add an integration-ish test**
  - Spawn MCP stdio stub, configure via `--mcp-config`-equivalent session KV, verify a tool call succeeds.

- [x] **Step 4: Run runtime tests**
  - Run: `cargo test -p rexos agent_loop -q`
  - Expected: PASS

---

## Chunk 5: Docs + verification

### Task 5: Make docs truthful and add quickstart snippet

**Files:**
- Modify: `docs-site/blog/mcp-integration-guide.md`
- Modify: `docs-site/reference/config.md` (optional: mention session flag, not TOML)

- [x] **Step 1: Update MCP blog to match real CLI**
  - Keep `--mcp-config` example, remove commands that do not exist (`config add-mcp-server`).

- [x] **Step 2: Verify docs build**
  - Run: `python3 -m mkdocs build --strict`
  - Expected: PASS

### Task 6: Full verification

- [x] **Step 1: Run full checks**
  - Run: `make check`
  - Expected: PASS

# Harness + Agent OS Alignment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Align RexOS with Anthropic’s “Effective harnesses for long-running agents” harness patterns (initializer + long-running coding harness) and with an Agent OS “core subsystems” baseline (tools, stability, extensibility), while keeping Ollama as the primary local smoke/E2E driver.

**Architecture:** Keep RexOS as a Cargo workspace. Expand `rexos-harness` from “scaffold + preflight” into a real harness runner:
- **Initializer**: generate a comprehensive `features.json` from a user prompt and initialize/verify workspace artifacts.
- **Runner**: run the coding agent with injected workspace context, then verify via `init.sh`, checkpoint via git commit, and retry with failure output when needed.
Add two runtime hardening pieces:
- `web_fetch` tool with SSRF protections.
- Tool-loop guard in the agent loop to prevent runaway repetition.

**Tech Stack:** Rust 2021, tokio, reqwest, axum (test mocks), rusqlite (bundled), clap, serde/toml/serde_json. Ollama used via OpenAI-compatible API for smoke tests only.

---

### Task 1: Add initializer harness (feature list generation)

**Goal:** Provide an initializer flow that creates the durable artifacts (`features.json`, `rexos-progress.md`, `init.sh`) and populates a **comprehensive** `features.json` from a user prompt (per Anthropic harness article).

**Files:**
- Modify: `crates/rexos-harness/src/lib.rs`
- Modify: `crates/loopforge-cli/src/main.rs`
- Test: `crates/rexos/tests/harness_initializer.rs`

**Step 1: Write failing test**

Create a mock OpenAI-compatible server that returns a `tool_calls` response instructing `fs_write` of `features.json` with at least one `passes: false` item. Then call the new harness initializer function and assert:
- `features.json` parses and contains at least 1 feature
- the workspace has a git commit after initialization

Run: `cargo test -p rexos harness_initializer`
Expected: FAIL (missing function / behavior)

**Step 2: Implement minimal initializer**

In `rexos-harness`, add a function like:
`bootstrap_with_prompt(agent: &AgentRuntime, workspace: &Path, session_id: &str, prompt: &str) -> Result<()>`

Behavior:
- If workspace not initialized: call `init_workspace()`
- Run `preflight()`
- Run `agent.run_session()` with an “initializer system prompt” that instructs generating `features.json`
- Run `init.sh` again (post-run verification)
- If git tree is dirty: commit a checkpoint (identity `RexOS <rexos@localhost>`)

**Step 3: Re-run test**

Expected: PASS

**Step 4: Wire CLI**

Extend `rexos harness init <dir>` with `--prompt <text>` and optional `--session`. If `--prompt` is provided, call the initializer flow using configured providers (Ollama default).

---

### Task 2: Add long-running harness runner (verify + checkpoint + retry)

**Goal:** Make `rexos harness run` an outer-loop harness, not a single call:
- Inject “get up to speed” context into the agent prompt (git log, progress tail, failing features summary)
- Run agent session
- Run `init.sh` afterwards
- If `init.sh` fails, feed the failure output back into the agent and retry (bounded attempts)
- If `init.sh` succeeds and git is dirty, auto-checkpoint commit

**Files:**
- Modify: `crates/rexos-harness/src/lib.rs`
- Modify: `crates/loopforge-cli/src/main.rs`
- Test: `crates/rexos/tests/harness_runner.rs`

**Step 1: Write failing test**

Mock server should:
- Call 1: write a file that causes `init.sh` to fail (e.g., `exit 1`), then stop
- Call 2: fix `init.sh` to succeed and write a marker file

Assert harness run returns successfully and the marker file exists, and that there is a new git commit after run.

Run: `cargo test -p rexos harness_runner`
Expected: FAIL

**Step 2: Implement runner**

Add function like:
`run_harness(agent: &AgentRuntime, workspace: &Path, session_id: &str, user_prompt: &str, max_attempts: usize) -> Result<String>`

**Step 3: Wire CLI**

Add flags to `rexos harness run`:
- `--prompt <text>` (required to actually run)
- `--session <id>` (optional; otherwise persistent session id per workspace)
- `--max-attempts <n>` (default 3)

---

### Task 3: Persist harness session id per workspace

**Goal:** By default, `rexos harness run` resumes the same session across invocations in the same workspace, without the user needing `--session`.

**Files:**
- Modify: `crates/rexos-harness/src/lib.rs`
- Test: `crates/rexos/tests/harness_session_persistence.rs`

**Step 1: Write failing test**

Initialize a workspace and call “resolve session id” twice; assert the second call returns the same id and that it is stored inside the workspace (e.g., `.rexos/session_id`).

**Step 2: Implement**

Store session id in `workspace/.rexos/session_id` (workspace-local, durable, git-ignored by default). Create directory if missing.

---

### Task 4: Add `web_fetch` tool with SSRF protections

**Goal:** Add a built-in `web_fetch` tool:
- Only `http`/`https`
- Deny loopback/private/link-local by default
- Size cap and timeout
- Return structured JSON string (`status`, `content_type`, `body`, `truncated`)

**Files:**
- Modify: `crates/rexos-tools/src/lib.rs`
- Modify: `crates/rexos-tools/Cargo.toml` (if new deps required)
- Test: `crates/rexos/tests/tools_web_fetch.rs`

**Step 1: Write failing tests**
- `web_fetch` rejects `file://` URLs
- `web_fetch` rejects `http://127.0.0.1` when `allow_private=false`
- `web_fetch` succeeds against a local mock server when `allow_private=true`

Run: `cargo test -p rexos tools_web_fetch`
Expected: FAIL

**Step 2: Implement tool**

Add to tool definitions + dispatch.

**Step 3: Re-run tests**

Expected: PASS

---

### Task 5: Add tool-loop guard in agent runtime

**Goal:** Prevent runaway repetition of identical tool calls by failing early with a clear error.

**Files:**
- Modify: `crates/rexos-runtime/src/lib.rs`
- Test: `crates/rexos/tests/loop_guard.rs`

**Step 1: Write failing test**

Mock OpenAI-compatible server returns the same tool call (`fs_read` with same args) repeatedly. Assert `run_session()` errors with “tool loop detected”.

Run: `cargo test -p rexos loop_guard`
Expected: FAIL

**Step 2: Implement**

Track tool call signatures within the session and bail when threshold reached (default 3).

**Step 3: Re-run test**

Expected: PASS

---

### Task 6: Add an explicit alignment checklist doc

**Goal:** Make “对齐” auditable: one file that maps Anthropic harness requirements + RexOS core subsystems to modules, with “Implemented / Partial / Planned” and pointers.

**Files:**
- Create: `docs/alignment.md`

**Step 1: Write doc**

Include:
- Anthropic harness checklist mapping to `rexos-harness` + CLI
- RexOS core subsystems mapping to crates + what’s intentionally out of scope (channels/skills/wire)

**Step 2: Commit**

---

### Global verification

Run:
- `./init.sh`
- `cargo test`

Expected: PASS

Commit messages:
- One commit per task (small, reviewable)

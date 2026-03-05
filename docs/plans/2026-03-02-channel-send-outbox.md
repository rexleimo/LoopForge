# Channel Send (Outbox + Dispatcher) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task.

**Goal:** Implement a safe `channel_send` tool that enqueues messages into an outbox, plus a dispatcher to drain the outbox via adapters (console + webhook) and a CLI workflow to run the dispatcher.

**Architecture:** `channel_send` is a runtime tool (not `Toolset`) that only persists an outbox record. A dispatcher drains queued records and performs delivery via channel adapters, updating status/attempts/error fields.

**Tech Stack:** Rust, SQLite KV store (`rexos-memory`), `reqwest` (webhook), clap (CLI).

---

### Task 1: Add a failing `channel_send` enqueue test

**Files:**
- Create: `crates/rexos/tests/reserved_channel_tools.rs`

**Step 1: Write the failing test**
- Use a mock OpenAI-compatible server (axum) that returns a tool call to `channel_send`.
- After `AgentRuntime::run_session`, assert `kv_get("rexos.outbox.messages")` contains one queued record.

**Step 2: Run the test to verify it fails**

Run: `cargo test -p rexos --test reserved_channel_tools`

Expected: FAIL with `tool not implemented yet: channel_send`.

---

### Task 2: Implement runtime `channel_send` + outbox storage

**Files:**
- Modify: `crates/rexos-runtime/src/lib.rs`
- Modify: `crates/rexos-tools/src/lib.rs` (tool definition + Toolset behavior)

**Step 1: Add runtime tool handling**
- Add a `channel_send` match arm in `AgentRuntime::run_session`.
- Persist `OutboxMessageRecord` into `MemoryStore` under `rexos.outbox.messages`.

**Step 2: Update tool definitions**
- Replace the stub `channel_send` tool definition with a real schema:
  - `channel`, `recipient`, `message` required
  - optional `subject`
- Mark `channel_send` as “implemented in runtime” for standalone `Toolset::call`.

**Step 3: Run the test to verify it passes**

Run: `cargo test -p rexos --test reserved_channel_tools`

Expected: PASS.

---

### Task 3: Implement dispatcher + adapters (console + webhook)

**Files:**
- Modify: `crates/rexos-runtime/Cargo.toml` (add `reqwest`)
- Modify: `crates/rexos-runtime/src/lib.rs` (dispatcher + adapters)
- Create/Modify: `crates/rexos/tests/channel_dispatcher.rs`

**Step 1: Write failing dispatcher test (console)**
- Enqueue a message via `channel_send`.
- Drain once via dispatcher.
- Assert outbox record is marked `sent`.

**Step 2: Minimal implementation**
- `console` adapter: mark as sent and print a structured line.
- `webhook` adapter: POST JSON to `REXOS_WEBHOOK_URL` (only if set).
- Update attempts + last_error on failures.

**Step 3: Run tests**

Run: `cargo test -p rexos --test channel_dispatcher`

Expected: PASS.

---

### Task 4: Add CLI command to drain/worker

**Files:**
- Modify: `crates/loopforge-cli/src/main.rs`
- Modify: `docs-site/reference/cli.md`
- Modify: `docs-site/reference/tools.md` + `docs-site/zh/reference/tools.md`

**Steps:**
- Add `rexos channel drain` to deliver queued messages once.
- Add `rexos channel worker --interval-secs N` loop (optional but recommended).
- Document how to use it and how to configure webhook via env.

**Verification:**
- `cargo test`
- `cargo run -p loopforge-cli -- channel drain` (manual smoke, optional)


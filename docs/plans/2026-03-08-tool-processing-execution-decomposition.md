# Tool Processing Execution Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/session_runner/tool_processing/execution.rs` into smaller focused helpers with zero behavior change.

**Architecture:** Keep `execution.rs` as the orchestration layer. Move whitelist/approval/start-event logic into `preflight.rs` and move leak-guard/result-audit completion logic into `outcome.rs`. Preserve ACP events, audit records, truncation, and error strings exactly.

**Tech Stack:** Rust, session runtime integration tests, `cargo fmt`.

---

### Task 1: Extract preflight helpers
1. Create `crates/rexos-runtime/src/session_runner/tool_processing/preflight.rs`.
2. Move session whitelist enforcement there.
3. Move approval warning and tool.started event emission there.

### Task 2: Extract outcome helpers
1. Create `crates/rexos-runtime/src/session_runner/tool_processing/outcome.rs`.
2. Move leak-guard inspection, failure/block handling, truncation, and success audit/event emission there.
3. Keep returned safe error text and tool chat payload behavior unchanged.

### Task 3: Verify no behavior change
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test runtime_controls -- --nocapture`.
3. Run `cargo test -p rexos --test agent_loop -- --nocapture`.
4. Run `cargo test --workspace --locked`.

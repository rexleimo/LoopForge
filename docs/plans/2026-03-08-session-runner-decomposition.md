# Session Runner Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/session_runner.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep `session_runner.rs` as the module boundary. Move session history/message bootstrap into `history.rs`, move provider/model/chat helpers into `llm.rs`, and move the main loop into `chat_loop.rs`. Preserve message ordering, session event timing, iteration limits, and tool-loop detection behavior exactly.

**Tech Stack:** Rust, OpenAI-compatible chat message types, runtime integration tests, `cargo fmt`.

---

### Task 1: Extract session history bootstrap
1. Create `crates/rexos-runtime/src/session_runner/history.rs`.
2. Move history loading plus system/user message seeding into a helper there.
3. Keep persisted message order unchanged.

### Task 2: Extract LLM helpers and loop
1. Create `crates/rexos-runtime/src/session_runner/llm.rs`.
2. Create `crates/rexos-runtime/src/session_runner/chat_loop.rs`.
3. Move `resolve_model` and `driver_chat` into `llm.rs` and move `run_session` into `chat_loop.rs`.

### Task 3: Verify no behavior change
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test agent_loop -- --nocapture`.
3. Run `cargo test -p rexos --test agent_default_model -- --nocapture`.
4. Run `cargo test --workspace --locked`.

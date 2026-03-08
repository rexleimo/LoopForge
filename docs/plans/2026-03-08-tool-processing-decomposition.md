# Session Runner Tool Processing Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/session_runner/tool_processing.rs` into smaller internal modules while preserving session lifecycle events, tool approval warnings, leak-guard handling, audit persistence, and returned tool messages exactly.

**Architecture:** Keep `tool_processing.rs` as a thin internal module boundary and move session event emitters plus tool-call processing logic into dedicated submodules. Preserve all current `AgentRuntime` method names, event types, payload fields, and error behavior.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split helper families
1. Move session and tool ACP event emitters into `crates/rexos-runtime/src/session_runner/tool_processing/events.rs`.
2. Move tool-call execution, leak-guard handling, and tool message creation into `crates/rexos-runtime/src/session_runner/tool_processing/execution.rs`.

## Task 2: Keep internal API stable
1. Preserve `append_session_started_event`, `append_session_completed_event`, `append_session_failed_event`, and `process_tool_call` names on `AgentRuntime`.
2. Keep `tool_processing.rs` as a module boundary with tests only.

## Task 3: Verify behavior
1. Run `cargo test -p rexos-runtime --lib tool_processing::tests -- --nocapture`.
2. Run `cargo test -p rexos --test runtime_controls -- --nocapture`.
3. Run `cargo test --workspace --locked`.

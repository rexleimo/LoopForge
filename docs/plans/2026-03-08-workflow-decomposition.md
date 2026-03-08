# Workflow Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/workflow.rs` into smaller internal modules while preserving workflow state persistence, approval warnings, tool execution, and ACP events exactly.

**Architecture:** Keep `workflow.rs` as a thin internal boundary and move pure workflow state construction/persistence and step execution/event helpers into dedicated submodules. Preserve all current `AgentRuntime` method names, JSON payload shapes, and error behavior.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split workflow state helpers
1. Move initial `WorkflowRunStateRecord` construction and JSON file persistence helpers into `crates/rexos-runtime/src/workflow/state.rs`.
2. Keep the same workflow id generation flow, pending step defaults, and file serialization format.

## Task 2: Split workflow execution helpers
1. Move per-step argument serialization, approval warning emission, success/failure bookkeeping, and final result formatting into `crates/rexos-runtime/src/workflow/execution.rs`.
2. Keep ACP event types, payload fields, truncation rules, and `continue_on_error` handling unchanged.

## Task 3: Verify behavior
1. Run `cargo test -p rexos --test runtime_controls workflow_run_persists_state_and_executes_steps -- --nocapture`.
2. Run `cargo test -p rexos --test runtime_controls`.
3. Run `cargo test --workspace --locked`.

# Approval Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/approval.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep `approval.rs` as the module boundary. Move env/permission parsing into `permissions.rs` and move tool approval gating plus `AgentRuntime::evaluate_tool_approval` into `tool_gate.rs`. Preserve env var names, read-only heuristics, approval warning text, and ACP blocked event payloads exactly.

**Tech Stack:** Rust, `serde_json`, runtime unit tests, `runtime_controls` integration tests.

---

### Task 1: Extract permission/env helpers
1. Create `crates/rexos-runtime/src/approval/permissions.rs`.
2. Move `tool_approval_is_granted`, `skill_approval_is_granted`, and `skill_permissions_are_readonly` there.
3. Keep env var names and normalization logic unchanged.

### Task 2: Extract tool approval gate
1. Create `crates/rexos-runtime/src/approval/tool_gate.rs`.
2. Move `ApprovalMode`, `tool_requires_approval`, json parsing helper, and `evaluate_tool_approval` there.
3. Keep blocked event payload and error message unchanged.

### Task 3: Verify no behavior change
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos-runtime --lib -- --nocapture`.
3. Run `cargo test -p rexos --test runtime_controls -- --nocapture`.
4. Run `cargo test --workspace --locked`.

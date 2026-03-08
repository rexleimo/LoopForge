# Scheduling Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/scheduling.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep `scheduling.rs` as the module boundary on `AgentRuntime`. Move schedule/cron persistence helpers into `storage.rs`, keep schedule operations in `schedules.rs`, and keep cron operations in `cron.rs`. Preserve storage keys, duplicate-id behavior, retention limits, and JSON responses exactly.

**Tech Stack:** Rust, `rexos-memory`, reserved tool integration tests, `cargo fmt`.

---

### Task 1: Extract schedule and cron storage helpers

**Files:**
- Update: `meos/crates/rexos-runtime/src/scheduling.rs`
- Create: `meos/crates/rexos-runtime/src/scheduling/storage.rs`

**Steps:**
1. Move `schedules_get` and `schedules_set` into `storage.rs`.
2. Move `cron_jobs_get` and `cron_jobs_set` into `storage.rs`.
3. Keep storage keys and serde fallback behavior unchanged.

### Task 2: Extract schedule and cron operations

**Files:**
- Update: `meos/crates/rexos-runtime/src/scheduling.rs`
- Create: `meos/crates/rexos-runtime/src/scheduling/schedules.rs`
- Create: `meos/crates/rexos-runtime/src/scheduling/cron.rs`

**Steps:**
1. Move `schedule_create`, `schedule_list`, and `schedule_delete` into `schedules.rs`.
2. Move `cron_create`, `cron_list`, and `cron_cancel` into `cron.rs`.
3. Keep retention limit `200`, `agent_id` fallback logic, and returned JSON unchanged.

### Task 3: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-runtime/src/scheduling/**/*.rs`
- Verify: `meos/crates/rexos/tests/reserved_schedule_tools.rs`
- Verify: `meos/crates/rexos/tests/reserved_cron_tools.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test reserved_schedule_tools -- --nocapture`.
3. Run `cargo test -p rexos --test reserved_cron_tools -- --nocapture`.
4. Run `cargo test --workspace --locked`.

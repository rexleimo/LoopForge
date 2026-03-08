# Daemon Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-daemon/src/lib.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `app`, `app_with_config`, and `serve` API stable while moving config loading, rate limiting, handlers, and middleware concerns into internal modules. Preserve auth checks, rate limiting, and security headers exactly.

**Tech Stack:** Rust, `axum`, `tokio`, existing `daemon_health` integration test.

---

### Task 1: Split config and state helpers

**Files:**
- Update: `meos/crates/rexos-daemon/src/lib.rs`
- Create: `meos/crates/rexos-daemon/src/config.rs`
- Create: `meos/crates/rexos-daemon/src/rate_limit.rs`
- Create: `meos/crates/rexos-daemon/src/state.rs`
- Create: `meos/crates/rexos-daemon/src/handlers.rs`
- Create: `meos/crates/rexos-daemon/src/middleware.rs`

**Steps:**
1. Move daemon config defaults into `config.rs`.
2. Move rate-limiter state into `rate_limit.rs`.
3. Move app state and response payload structs into `state.rs`.
4. Move route handlers and middleware helpers into dedicated modules.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-daemon/src/*.rs`
- Verify: `meos/crates/rexos/tests/daemon_health.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test daemon_health -- --nocapture`.
3. Run `cargo test -p rexos-daemon --lib`.
4. Run `cargo test --workspace --locked`.

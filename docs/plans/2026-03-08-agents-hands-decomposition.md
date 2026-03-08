# Agents and Hands Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/agents_hands/agents.rs` and `crates/rexos-runtime/src/agents_hands/hands.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the `AgentRuntime` API stable while moving shared persistence helpers, hand definitions, and lifecycle/send operations into dedicated internal submodules. Preserve storage keys, JSON payloads, and status transitions exactly.

**Tech Stack:** Rust, `rexos-memory`, existing reserved tool integration tests.

---

### Task 1: Split agent persistence and lifecycle

**Files:**
- Update: `meos/crates/rexos-runtime/src/agents_hands/agents.rs`
- Create: `meos/crates/rexos-runtime/src/agents_hands/agents/storage.rs`
- Create: `meos/crates/rexos-runtime/src/agents_hands/agents/lifecycle.rs`

**Steps:**
1. Move agent index/key/load/store helpers into `storage.rs`.
2. Move spawn/list/find/kill/send logic into `lifecycle.rs`.
3. Keep `agents.rs` as a thin module boundary.

### Task 2: Split hand definitions, persistence, and lifecycle

**Files:**
- Update: `meos/crates/rexos-runtime/src/agents_hands/hands.rs`
- Create: `meos/crates/rexos-runtime/src/agents_hands/hands/defs.rs`
- Create: `meos/crates/rexos-runtime/src/agents_hands/hands/storage.rs`
- Create: `meos/crates/rexos-runtime/src/agents_hands/hands/lifecycle.rs`

**Steps:**
1. Move static hand definitions into `defs.rs`.
2. Move hand index/key/load/store helpers into `storage.rs`.
3. Move list/activate/status/deactivate logic into `lifecycle.rs`.

### Task 3: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-runtime/src/agents_hands/**/*.rs`
- Verify: `meos/crates/rexos/tests/reserved_agent_tools.rs`
- Verify: `meos/crates/rexos/tests/reserved_hand_tools.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test reserved_agent_tools -- --nocapture`.
3. Run `cargo test -p rexos --test reserved_hand_tools -- --nocapture`.
4. Run `cargo test --workspace --locked`.

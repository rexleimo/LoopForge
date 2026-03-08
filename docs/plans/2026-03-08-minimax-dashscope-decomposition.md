# MiniMax and Dashscope Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/minimax.rs` and `crates/rexos-llm/src/dashscope.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `MiniMaxDriver` and `DashscopeDriver` entry points stable while moving request/response wire types and message-mapping helpers into internal modules. Preserve request payload shapes, cleanup rules, and legacy `function_call` compatibility exactly.

**Tech Stack:** Rust, `reqwest`, `serde`, existing `llm_minimax` and `llm_dashscope` integration tests.

---

### Task 1: Split MiniMax layers

**Files:**
- Update: `meos/crates/rexos-llm/src/minimax.rs`
- Create: `meos/crates/rexos-llm/src/minimax/driver.rs`
- Create: `meos/crates/rexos-llm/src/minimax/types.rs`
- Create: `meos/crates/rexos-llm/src/minimax/mapping.rs`

**Steps:**
1. Move MiniMax request/response wire structs into `types.rs`.
2. Move raw response mapping into `mapping.rs`.
3. Keep HTTP driver orchestration in `driver.rs`.

### Task 2: Split Dashscope layers

**Files:**
- Update: `meos/crates/rexos-llm/src/dashscope.rs`
- Create: `meos/crates/rexos-llm/src/dashscope/driver.rs`
- Create: `meos/crates/rexos-llm/src/dashscope/types.rs`
- Create: `meos/crates/rexos-llm/src/dashscope/mapping.rs`

**Steps:**
1. Move Dashscope request/response wire structs into `types.rs`.
2. Move request/cleanup helpers into `mapping.rs`.
3. Keep HTTP driver orchestration in `driver.rs`.

### Task 3: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-llm/src/minimax*.rs`
- Verify: `meos/crates/rexos-llm/src/dashscope*.rs`
- Verify: `meos/crates/rexos/tests/llm_minimax.rs`
- Verify: `meos/crates/rexos/tests/llm_dashscope.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test llm_minimax -- --nocapture`.
3. Run `cargo test -p rexos --test llm_dashscope -- --nocapture`.
4. Run `cargo test -p rexos-llm --lib`.
5. Run `cargo test --workspace --locked`.

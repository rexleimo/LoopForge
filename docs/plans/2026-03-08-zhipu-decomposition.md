# Zhipu Driver Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/zhipu.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `rexos_llm::zhipu::ZhipuDriver` entry point stable while moving request/response wire structs, response mapping, and JWT/bearer-token helpers into separate internal modules. Preserve JWT signing shape, timeout behavior, and legacy `function_call` compatibility exactly.

**Tech Stack:** Rust, `reqwest`, `serde`, `hmac`, `sha2`, existing `llm_zhipu` integration test.

---

### Task 1: Split auth, mapping, and driver layers

**Files:**
- Update: `meos/crates/rexos-llm/src/zhipu.rs`
- Create: `meos/crates/rexos-llm/src/zhipu/driver.rs`
- Create: `meos/crates/rexos-llm/src/zhipu/types.rs`
- Create: `meos/crates/rexos-llm/src/zhipu/mapping.rs`
- Create: `meos/crates/rexos-llm/src/zhipu/auth.rs`

**Steps:**
1. Move Zhipu request/response wire structs into `types.rs`.
2. Move raw message mapping into `mapping.rs`.
3. Move bearer-token and JWT helpers into `auth.rs`.
4. Keep `ZhipuDriver` in `driver.rs` and re-export it from `zhipu.rs`.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-llm/src/zhipu*.rs`
- Verify: `meos/crates/rexos/tests/llm_zhipu.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test llm_zhipu -- --nocapture`.
3. Run `cargo test -p rexos-llm --lib`.
4. Run `cargo test --workspace --locked`.

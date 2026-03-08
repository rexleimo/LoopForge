# Anthropic Driver Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/anthropic.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `rexos_llm::anthropic::AnthropicDriver` entry point stable while moving Anthropic wire types, request mapping, response mapping, and HTTP driver logic into separate internal modules. Preserve system/message/tool translation and error messages exactly.

**Tech Stack:** Rust, `reqwest`, `serde`, existing `llm_anthropic` integration test.

---

### Task 1: Split driver and mapping layers

**Files:**
- Update: `meos/crates/rexos-llm/src/anthropic.rs`
- Create: `meos/crates/rexos-llm/src/anthropic/driver.rs`
- Create: `meos/crates/rexos-llm/src/anthropic/types.rs`
- Create: `meos/crates/rexos-llm/src/anthropic/request.rs`
- Create: `meos/crates/rexos-llm/src/anthropic/response.rs`

**Steps:**
1. Move Anthropic wire structs into `types.rs`.
2. Move tool/message request mapping into `request.rs`.
3. Move response-to-OpenAI-compat mapping into `response.rs`.
4. Keep `AnthropicDriver` in `driver.rs` and re-export it from `anthropic.rs`.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-llm/src/anthropic*.rs`
- Verify: `meos/crates/rexos/tests/llm_anthropic.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test llm_anthropic -- --nocapture`.
3. Run `cargo test -p rexos-llm --lib`.
4. Run `cargo test --workspace --locked`.

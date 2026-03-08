# Gemini Driver Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/gemini.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `rexos_llm::gemini::GeminiDriver` entry point stable while moving Gemini wire types, request mapping, response mapping, and driver HTTP logic into separate internal modules. Preserve role mapping, tool-call serialization, and error messages exactly.

**Tech Stack:** Rust, `reqwest`, `serde`, existing `llm_gemini` integration test.

---

### Task 1: Split driver and wire types

**Files:**
- Update: `meos/crates/rexos-llm/src/gemini.rs`
- Create: `meos/crates/rexos-llm/src/gemini/driver.rs`
- Create: `meos/crates/rexos-llm/src/gemini/types.rs`
- Create: `meos/crates/rexos-llm/src/gemini/request.rs`
- Create: `meos/crates/rexos-llm/src/gemini/response.rs`

**Steps:**
1. Move Gemini request/response wire structs into `types.rs`.
2. Move request mapping (`map_tools`, `map_messages`) into `request.rs`.
3. Move response mapping into `response.rs`.
4. Keep `GeminiDriver` in `driver.rs` and re-export it from `gemini.rs`.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-llm/src/gemini*.rs`
- Verify: `meos/crates/rexos/tests/llm_gemini.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test llm_gemini -- --nocapture`.
3. Run `cargo test -p rexos-llm --lib`.
4. Run `cargo test --workspace --locked`.

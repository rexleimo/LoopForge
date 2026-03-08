# OpenAI Compat Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/openai_compat.rs` into focused modules with zero API and behavior change.

**Architecture:** Preserve the public `rexos_llm::openai_compat::*` surface while moving public request/response types, raw-response mapping, retry/timeouts, and the HTTP client implementation into separate internal modules. Keep retry policy, timeout env vars, and legacy `function_call` compatibility unchanged.

**Tech Stack:** Rust, `reqwest`, `serde`, existing integration tests in `crates/rexos/tests/llm_openai_compat.rs`.

---

### Task 1: Split public types from client logic

**Files:**
- Update: `meos/crates/rexos-llm/src/openai_compat.rs`
- Create: `meos/crates/rexos-llm/src/openai_compat/types.rs`
- Create: `meos/crates/rexos-llm/src/openai_compat/client.rs`
- Create: `meos/crates/rexos-llm/src/openai_compat/mapping.rs`
- Create: `meos/crates/rexos-llm/src/openai_compat/retry.rs`

**Steps:**
1. Move public request/response types into `types.rs`.
2. Move raw response decoding and `function_call` fallback mapping into `mapping.rs`.
3. Move timeout/retry helpers into `retry.rs`.
4. Keep `OpenAiCompatibleClient` in `client.rs` and re-export everything from the root module.

### Task 2: Verify compatibility

**Files:**
- Verify: `meos/crates/rexos-llm/src/openai_compat*.rs`
- Verify: `meos/crates/rexos/tests/llm_openai_compat.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test llm_openai_compat -- --nocapture`.
3. Run `cargo test -p rexos-llm --lib`.
4. Run `cargo test --workspace --locked`.

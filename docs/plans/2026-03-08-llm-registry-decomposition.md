# LLM Registry Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-llm/src/registry.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public `LlmRegistry` type and methods stable while moving provider-driver construction logic and tests into dedicated internal modules. Preserve provider-kind routing and default-model lookup behavior exactly.

**Tech Stack:** Rust, existing LLM drivers, `rexos-kernel` config types.

---

### Task 1: Split registry construction logic

**Files:**
- Update: `meos/crates/rexos-llm/src/registry.rs`
- Create: `meos/crates/rexos-llm/src/registry/build.rs`
- Create: `meos/crates/rexos-llm/src/registry/tests.rs`

**Steps:**
1. Keep `LlmRegistry` definition and lightweight accessors in `registry.rs`.
2. Move `from_config` and provider-driver construction into `build.rs`.
3. Move existing tests into `tests.rs` unchanged except for imports.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-llm/src/registry*.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos-llm --lib -- --nocapture`.
3. Run `cargo test --workspace --locked`.

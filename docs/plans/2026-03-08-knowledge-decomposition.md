# Knowledge Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-runtime/src/knowledge.rs` into smaller focused internal modules with zero behavior change.

**Architecture:** Keep `knowledge.rs` as the module boundary on `AgentRuntime`. Move entity/relation persistence helpers into `storage.rs` and keep add/query operations in dedicated operation modules. Preserve storage keys, JSON shapes, and duplicate-id behavior exactly.

**Tech Stack:** Rust, `rexos-memory`, reserved tool integration tests, `cargo fmt`.

---

### Task 1: Extract knowledge storage helpers

**Files:**
- Update: `meos/crates/rexos-runtime/src/knowledge.rs`
- Create: `meos/crates/rexos-runtime/src/knowledge/storage.rs`

**Steps:**
1. Move entity list get/set helpers into `storage.rs`.
2. Move relation list get/set helpers into `storage.rs`.
3. Keep storage keys and serde fallback behavior unchanged.

### Task 2: Extract knowledge operations

**Files:**
- Update: `meos/crates/rexos-runtime/src/knowledge.rs`
- Create: `meos/crates/rexos-runtime/src/knowledge/mutations.rs`
- Create: `meos/crates/rexos-runtime/src/knowledge/query.rs`

**Steps:**
1. Move `knowledge_add_entity` and `knowledge_add_relation` into `mutations.rs`.
2. Move `knowledge_query` into `query.rs`.
3. Keep query matching rules and returned JSON unchanged.

### Task 3: Verify no behavior change

**Files:**
- Verify: `meos/crates/rexos-runtime/src/knowledge/**/*.rs`
- Verify: `meos/crates/rexos/tests/reserved_knowledge_tools.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos --test reserved_knowledge_tools -- --nocapture`.
3. Run `cargo test --workspace --locked`.

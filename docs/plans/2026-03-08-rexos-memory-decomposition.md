# Rexos Memory Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/rexos-memory/src/lib.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public crate API unchanged while moving migration, KV storage, message persistence, chat-message conversion, and time helpers into bounded internal modules. Reuse the existing unit tests as the behavior guardrail and only widen visibility where internal sibling modules require it.

**Tech Stack:** Rust, `rusqlite`, `anyhow`, existing `rexos_llm` OpenAI-compatible types.

---

### Task 1: Define target module map

**Files:**
- Update: `meos/crates/rexos-memory/src/lib.rs`
- Create: `meos/crates/rexos-memory/src/store.rs`
- Create: `meos/crates/rexos-memory/src/schema.rs`
- Create: `meos/crates/rexos-memory/src/kv.rs`
- Create: `meos/crates/rexos-memory/src/messages.rs`
- Create: `meos/crates/rexos-memory/src/chat.rs`
- Create: `meos/crates/rexos-memory/src/time.rs`
- Create: `meos/crates/rexos-memory/src/tests.rs`

**Steps:**
1. Reduce `lib.rs` to module declarations plus shared type definitions.
2. Preserve the `MemoryStore` and `Message` public shapes exactly.
3. Keep internal visibility as narrow as possible while allowing sibling-module impl blocks.

### Task 2: Move store open/migration logic

**Files:**
- Update: `meos/crates/rexos-memory/src/lib.rs`
- Create: `meos/crates/rexos-memory/src/store.rs`
- Create: `meos/crates/rexos-memory/src/schema.rs`

**Steps:**
1. Move open/create entry points into `store.rs`.
2. Move SQL schema/bootstrap logic into `schema.rs`.
3. Keep migration SQL and backfill behavior byte-for-byte equivalent.

### Task 3: Move KV and raw message persistence

**Files:**
- Create: `meos/crates/rexos-memory/src/kv.rs`
- Create: `meos/crates/rexos-memory/src/messages.rs`

**Steps:**
1. Extract `kv_set` and `kv_get` into a dedicated KV module.
2. Extract `append_message` and `list_messages` into a message persistence module.
3. Keep SQL, ordering, and returned structures unchanged.

### Task 4: Move chat conversion helpers and tests

**Files:**
- Create: `meos/crates/rexos-memory/src/chat.rs`
- Create: `meos/crates/rexos-memory/src/time.rs`
- Create: `meos/crates/rexos-memory/src/tests.rs`

**Steps:**
1. Extract chat-message conversion and role mapping helpers.
2. Move the clock helper to a tiny shared module.
3. Move existing unit tests unchanged except for imports.

### Task 5: Verify zero-behavior refactor

**Files:**
- Verify: `meos/crates/rexos-memory/src/*.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos-memory -- --nocapture`.
3. Run `cargo test --workspace --locked`.

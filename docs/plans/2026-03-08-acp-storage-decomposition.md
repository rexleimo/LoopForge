# ACP Storage Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically split `crates/rexos-runtime/src/acp.rs` into smaller event/checkpoint storage modules with zero behavior change.

**Architecture:** Keep the `acp` module boundary stable while moving event-list persistence and delivery-checkpoint persistence into separate child modules. Preserve constants, key formats, truncation behavior, and existing unit tests exactly.

**Tech Stack:** Rust, `rexos-memory`, serde JSON persistence.

---

### Task 1: Separate event and checkpoint stores

**Files:**
- Update: `meos/crates/rexos-runtime/src/acp.rs`
- Create: `meos/crates/rexos-runtime/src/acp/events.rs`
- Create: `meos/crates/rexos-runtime/src/acp/checkpoints.rs`
- Create: `meos/crates/rexos-runtime/src/acp/tests.rs`

**Steps:**
1. Move ACP event get/set/append logic into `events.rs`.
2. Move delivery checkpoint key/get/set logic into `checkpoints.rs`.
3. Keep `acp.rs` as the thin boundary module and test host.

### Task 2: Verify runtime behavior

**Files:**
- Verify: `meos/crates/rexos-runtime/src/acp*.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p rexos-runtime acp::tests -- --nocapture`.
3. Run `cargo test -p rexos-runtime --lib`.
4. Run `cargo test --workspace --locked`.

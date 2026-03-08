# Rexos Runtime Lib Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/lib.rs` into smaller internal modules while preserving runtime construction, audit persistence, ACP listing, approval evaluation, and utility helper behavior exactly.

**Architecture:** Keep `lib.rs` as the crate root that defines `AgentRuntime`, constants, and core exports. Move audit/ACP helper methods and small utility helpers into dedicated internal modules, preserving current method names, event payloads, storage keys, and approval behavior.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split runtime helper families
1. Move tool/skill audit persistence plus ACP listing helpers into `crates/rexos-runtime/src/runtime_state.rs`.
2. Move tool event payload construction, workflow path helper, and runtime-managed tool classification into `crates/rexos-runtime/src/runtime_utils.rs`.

## Task 2: Keep root API stable
1. Leave `AgentRuntime` struct, constructors, and `now_epoch_seconds` in `crates/rexos-runtime/src/lib.rs`.
2. Rewire existing internal call sites to use the moved helpers without changing signatures or behavior.

## Task 3: Verify behavior
1. Run `cargo test -p rexos-runtime --lib runtime_state::tests -- --nocapture`.
2. Run `cargo test -p rexos --test runtime_controls -- --nocapture`.
3. Run `cargo test --workspace --locked`.

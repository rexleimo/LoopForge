# Outbox Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/outbox.rs` by extracting duplicated outbox storage helpers and isolating delivery-specific helpers without changing queueing or dispatch behavior.

**Architecture:** Keep `outbox.rs` as the public module boundary that exports `OutboxDispatcher` and `OutboxDrainSummary`. Move shared persistence and delivery helpers into internal submodules so both `OutboxDispatcher` and `AgentRuntime::channel_send` reuse the same code path.

**Tech Stack:** Rust workspace, `reqwest`, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Extract shared outbox storage helpers
1. Create an internal store helper module under `crates/rexos-runtime/src/outbox/`.
2. Move outbox message get/set logic there so `OutboxDispatcher` and `AgentRuntime` stop duplicating the same persistence code.
3. Keep the storage key and retention behavior unchanged.

## Task 2: Extract delivery helpers
1. Create an internal delivery helper module under `crates/rexos-runtime/src/outbox/`.
2. Move webhook/console delivery helpers and ACP checkpoint update logic there.
3. Keep payloads, checkpoint keys, and console output format unchanged.

## Task 3: Verify behavior
1. Run `cargo test -p rexos --test reserved_channel_tools`.
2. Run `cargo test -p rexos --test channel_dispatcher`.
3. Run `cargo test -p rexos --test runtime_controls`.
4. Run `cargo test --workspace --locked`.

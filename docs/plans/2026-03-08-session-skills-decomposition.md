# Session Skills Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/session_skills.rs` into smaller internal modules while preserving session allowlists, policy loading, and skill audit/event behavior exactly.

**Architecture:** Keep `session_skills.rs` as a thin internal module boundary and split its concerns into storage helpers and skill audit/authorization helpers. Preserve all current `AgentRuntime` method names and their side effects.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split storage helpers
1. Move name normalization and session key/value load/store helpers into `crates/rexos-runtime/src/session_skills/storage.rs`.
2. Keep the same key format, JSON encoding, and default handling.

## Task 2: Split audit and authorization helpers
1. Move skill discovery, authorization, and execution audit methods into `crates/rexos-runtime/src/session_skills/audit.rs`.
2. Keep ACP event payloads, approval checks, and audit records unchanged.

## Task 3: Verify behavior
1. Run `cargo test -p rexos --test runtime_skills_policy`.
2. Run `cargo test -p rexos --test skills_audit_events`.
3. Run `cargo test --workspace --locked`.

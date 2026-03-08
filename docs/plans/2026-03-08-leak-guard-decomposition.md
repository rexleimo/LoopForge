# Leak Guard Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/leak_guard.rs` into smaller internal modules while preserving leak detection, redaction, and audit behavior exactly.

**Architecture:** Keep `leak_guard.rs` as the public module boundary and move pure helpers into submodules by concern: detectors, matching/redaction utilities, and tests. Avoid API changes outside the module and preserve all existing test expectations.

**Tech Stack:** Rust workspace, `serde`, `anyhow`, `cargo test`, targeted `rustfmt`.

## Task 1: Map leak-guard seams
1. Identify public types versus pure helper functions inside `crates/rexos-runtime/src/leak_guard.rs`.
2. Group logic into stable mechanical seams such as detector construction, content scanning, and redaction formatting.
3. Keep all exported types and methods reachable from the existing `crate::leak_guard` module path.

## Task 2: Extract helpers without behavior change
1. Turn `crates/rexos-runtime/src/leak_guard.rs` into a thin module root with submodules under `crates/rexos-runtime/src/leak_guard/`.
2. Move detector and regex-like pattern construction into one helper module.
3. Move match merging and redaction rendering into one helper module.
4. Move the large embedded test module into `crates/rexos-runtime/src/leak_guard/tests.rs`.

## Task 3: Verify behavior
1. Run `cargo test -p rexos-runtime --lib leak_guard -- --nocapture`.
2. Run `cargo test -p rexos --test runtime_controls`.
3. Run `cargo test --workspace --locked`.
4. Summarize the file-size reduction and exact files touched.

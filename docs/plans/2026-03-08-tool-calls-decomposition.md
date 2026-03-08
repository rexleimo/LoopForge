# Tool Calls Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-runtime/src/tool_calls.rs` into smaller internal modules while preserving tool-argument normalization, JSON tool-call extraction, and truncation behavior exactly.

**Architecture:** Keep `tool_calls.rs` as a thin internal boundary and move parsing helpers plus truncation helpers into dedicated submodules. Preserve current function names, fallback behavior, JSON shapes, and truncation marker text.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split helper families
1. Move tool-call parsing and nested-argument normalization into `crates/rexos-runtime/src/tool_calls/parse.rs`.
2. Move result truncation into `crates/rexos-runtime/src/tool_calls/truncate.rs`.

## Task 2: Keep internal API stable
1. Re-export `normalize_tool_arguments`, `parse_tool_calls_from_json_content`, and `truncate_tool_result_with_flag` from `crates/rexos-runtime/src/tool_calls.rs`.
2. Move tests into `crates/rexos-runtime/src/tool_calls/tests.rs`.

## Task 3: Verify behavior
1. Run `cargo test -p rexos-runtime --lib tool_calls::tests -- --nocapture`.
2. Run `cargo test -p rexos --test runtime_controls -- --nocapture`.
3. Run `cargo test --workspace --locked`.

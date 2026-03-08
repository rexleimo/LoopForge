# Rexos Harness Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-harness/src/lib.rs` into smaller internal modules while preserving workspace initialization, session persistence, feature normalization, init-script execution, and git checkpoint behavior exactly.

**Architecture:** Keep `lib.rs` as the public orchestration boundary and move git helpers, feature/document helpers, init-script execution, and prompt strings into dedicated internal modules. Preserve all current public function names, filesystem layout, git commands, and error messages.

**Tech Stack:** Rust workspace, `serde_json`, `cargo test`, targeted `rustfmt`.

## Task 1: Split helper families
1. Move git-oriented helpers into `crates/rexos-harness/src/git.rs`.
2. Move feature/progress parsing helpers into `crates/rexos-harness/src/features.rs`.
3. Move init-script selection/execution into `crates/rexos-harness/src/scripts.rs`.
4. Move harness system prompts into `crates/rexos-harness/src/prompts.rs`.

## Task 2: Keep public orchestration thin
1. Leave `init_workspace`, `resolve_session_id`, `bootstrap_with_prompt`, `run_harness`, and `preflight` in `crates/rexos-harness/src/lib.rs`.
2. Rewire those functions to call the new helpers without changing side effects.

## Task 3: Verify behavior
1. Run `cargo test -p rexos --test harness_init -- --nocapture`.
2. Run `cargo test -p rexos --test harness_initializer -- --nocapture`.
3. Run `cargo test -p rexos --test harness_runner -- --nocapture`.
4. Run `cargo test -p rexos --test harness_session_persistence -- --nocapture`.
5. Run `cargo test --workspace --locked`.

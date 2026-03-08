# Onboard Hotspots Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/loopforge-cli/src/onboard.rs` into a thinner orchestrator while keeping onboarding behavior and CLI output unchanged.

**Architecture:** Keep `onboard.rs` as the public module boundary and extract mechanical concerns into submodules: startup/runtime preparation, doctor blocking rules, and tests. Preserve all current public re-exports so callers and tests do not need broader changes.

**Tech Stack:** Rust workspace, `anyhow`, `serde`, `cargo test`, targeted `rustfmt`.

## Task 1: Split embedded onboarding tests
1. Move the large `#[cfg(test)]` block from `crates/loopforge-cli/src/onboard.rs` into `crates/loopforge-cli/src/onboard/tests.rs`.
2. Keep test imports minimal and preserve all current assertions.
3. Run `cargo test -p loopforge-cli --bin loopforge onboard::tests` or the closest focused test filter.

## Task 2: Extract onboarding flow helpers
1. Move `maybe_select_ollama_model` and `is_onboard_blocking_doctor_error` into dedicated helper modules under `crates/loopforge-cli/src/onboard/`.
2. Extract report-building and first-agent-run orchestration into a small `flow.rs` helper if it reduces `onboard.rs` materially without changing behavior.
3. Keep `run(...)` in `onboard.rs` as the top-level entrypoint.

## Task 3: Verify the split
1. Run `cargo test -p loopforge-cli --bin loopforge`.
2. Run `cargo test -p rexos --test runtime_controls` as a nearby integration safety check.
3. Run `cargo test --workspace --locked`.
4. Summarize the final file-size reduction and exact files touched.

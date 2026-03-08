# Runtime Hotspots Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Keep reducing `meos` runtime readability hotspots by splitting large record and runtime modules into narrower domain files without changing behavior.

**Architecture:** Preserve the existing public `rexos-runtime` API surface while turning large “mixed concerns” files into thin module boundaries. Prefer internal-only submodules under the existing filenames so external call sites keep importing the same symbols.

**Tech Stack:** Rust workspace, `serde`, `anyhow`, `cargo test`, targeted `rustfmt`.

## Task 1: Split runtime record definitions
1. Turn `crates/rexos-runtime/src/records.rs` into a thin re-export boundary.
2. Create domain files under `crates/rexos-runtime/src/records/` for memory, agents/hands, tasks/events, scheduling, outbox, audit, workflow, ACP, and knowledge records.
3. Keep visibility identical to today: `pub` only for exported ACP records and `SessionSkillPolicy`, `pub(crate)` for internal tool args and records.
4. Run `cargo test -p rexos-runtime --lib` and fix any import or visibility regressions.

## Task 2: Split agent and hand runtime logic
1. Turn `crates/rexos-runtime/src/agents_hands.rs` into a thin module root.
2. Move agent lifecycle and nested-session dispatch into `crates/rexos-runtime/src/agents_hands/agents.rs`.
3. Move hand catalog and hand instance persistence into `crates/rexos-runtime/src/agents_hands/hands.rs`.
4. Run `cargo test -p rexos-runtime --lib` and confirm behavior stays green.

## Task 3: Tackle the next bounded hotspot only if verification stays green
1. Inspect `crates/loopforge-cli/src/doctor/probes.rs` and identify natural seams such as config checks, provider checks, and browser checks.
2. Split only if the refactor stays mechanical and does not expand scope into behavior changes.
3. Re-run the closest CLI and integration tests after any additional split.

## Task 4: Final verification
1. Run `cargo test -p rexos-runtime --lib`.
2. Run `cargo test -p rexos --test runtime_controls`.
3. Run `cargo test --workspace --locked`.
4. Summarize the readability gains with exact files touched before handoff or commit.

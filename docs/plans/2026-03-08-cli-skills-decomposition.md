# CLI Skills Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/loopforge-cli/src/skills.rs` into smaller internal modules while preserving skill discovery, entry reading, permissions-to-tools mapping, and skills doctor behavior exactly.

**Architecture:** Keep `skills.rs` as the public CLI-facing boundary for skill types and re-exports. Move discovery/read helpers, permission mapping, and doctor filesystem inspection into dedicated internal modules with unchanged public function names.

**Tech Stack:** Rust workspace, `serde`, `cargo test`, targeted `rustfmt`.

## Task 1: Split helper families
1. Move discovery, list/find, source labeling, and entry reading helpers into `crates/loopforge-cli/src/skills/discovery.rs`.
2. Move permission-to-tool mapping into `crates/loopforge-cli/src/skills/permissions.rs`.
3. Move doctor root inspection into `crates/loopforge-cli/src/skills/doctor.rs`.

## Task 2: Keep public API stable
1. Leave `SkillListItem`, `SkillsDoctorLevel`, `SkillsDoctorIssue`, and `SkillsDoctorReport` in `crates/loopforge-cli/src/skills.rs`.
2. Re-export the same public helper functions from the new modules without changing their signatures or output.

## Task 3: Verify behavior
1. Run `cargo test -p loopforge-cli --bin loopforge skills::tests -- --nocapture`.
2. Run `cargo test -p loopforge-cli --bin loopforge`.
3. Run `cargo test --workspace --locked`.

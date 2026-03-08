# Doctor Config Probe Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/loopforge-cli/src/doctor/probes/config.rs` into smaller internal modules while preserving doctor config-path checks, provider/router/security diagnostics, and local Ollama probe behavior exactly.

**Architecture:** Keep `config.rs` as the probe boundary and move config-loading/path checks plus runtime/provider/security/Ollama checks into dedicated internal submodules. Preserve the same check ids, statuses, and message shapes.

**Tech Stack:** Rust workspace, `reqwest`, `cargo test`, targeted `rustfmt`.

## Task 1: Split helper families
1. Move config loading and path checks into `crates/loopforge-cli/src/doctor/probes/config/load.rs`.
2. Move router/provider/security/Ollama runtime checks into `crates/loopforge-cli/src/doctor/probes/config/runtime.rs`.

## Task 2: Keep public probe API stable
1. Preserve `load_config_checks` and `push_runtime_checks` signatures in the `config` probe boundary.
2. Keep all existing check ids/messages/status rules unchanged.

## Task 3: Verify behavior
1. Run `cargo test -p loopforge-cli --bin loopforge doctor::tests -- --nocapture`.
2. Run `cargo test -p loopforge-cli --bin loopforge`.
3. Run `cargo test --workspace --locked`.

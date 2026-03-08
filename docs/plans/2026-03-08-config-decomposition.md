# Rexos Kernel Config Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/rexos-kernel/src/config.rs` into smaller internal modules while preserving provider presets, default router behavior, config serialization, config loading, and skills-table compatibility exactly.

**Architecture:** Keep `config.rs` as the public type-definition boundary and move default preset construction plus file serialization/loading helpers into dedicated submodules. Preserve all public types and methods, existing TOML shapes, default values, and error behavior.

**Tech Stack:** Rust workspace, `toml`, `serde`, `cargo test`, targeted `rustfmt`.

## Task 1: Split defaults and storage helpers
1. Move provider preset and router/default helper construction into `crates/rexos-kernel/src/config/defaults.rs`.
2. Move config serialization/loading helpers into `crates/rexos-kernel/src/config/storage.rs`.

## Task 2: Keep public API stable
1. Leave the public config structs/enums in `crates/rexos-kernel/src/config.rs`.
2. Rewire `Default` impls and `RexosConfig::{ensure_default, load, load_skills_config}` to delegate to the new helpers without behavior changes.

## Task 3: Verify behavior
1. Run `cargo test -p rexos-kernel config::tests -- --nocapture`.
2. Run `cargo test --workspace --locked`.

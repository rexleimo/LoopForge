# LoopForge CLI Commands Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/loopforge-cli/src/cli/commands.rs` into smaller subcommand-focused modules with zero CLI behavior change.

**Architecture:** Keep `Cli` and top-level `Command` in `commands.rs`, and move domain-specific nested command enums into `cli/commands/*.rs`. Preserve all clap annotations, defaults, and enum names exactly so parsing behavior and downstream dispatch imports stay unchanged.

**Tech Stack:** Rust, `clap`, existing `loopforge-cli` dispatch layer.

---

### Task 1: Define subcommand module boundaries

**Files:**
- Update: `meos/crates/loopforge-cli/src/cli/commands.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/acp.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/agent.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/channel.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/config.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/daemon.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/harness.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/release.rs`
- Create: `meos/crates/loopforge-cli/src/cli/commands/skills.rs`

**Steps:**
1. Keep the top-level parser in `commands.rs`.
2. Move each nested subcommand enum into its own module.
3. Re-export the moved enums from `commands.rs` so public crate imports stay stable.

### Task 2: Verify clap behavior is unchanged

**Files:**
- Verify: `meos/crates/loopforge-cli/src/cli/tests.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p loopforge-cli --bin loopforge cli::tests -- --nocapture`.
3. Run `cargo test -p loopforge-cli --bin loopforge`.
4. Run `cargo test --workspace --locked`.

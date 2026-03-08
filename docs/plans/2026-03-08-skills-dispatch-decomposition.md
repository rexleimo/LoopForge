# Skills Dispatch Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/loopforge-cli/src/dispatch/skills.rs` into smaller focused modules with zero behavior change.

**Architecture:** Keep the public dispatch entrypoint stable while moving list/show rendering, doctor reporting, and skill execution flow into dedicated internal modules. Preserve printed output, session setup, and exit-code behavior exactly.

**Tech Stack:** Rust, existing `loopforge-cli` runtime/skills modules.

---

### Task 1: Split list/show and doctor paths

**Files:**
- Update: `meos/crates/loopforge-cli/src/dispatch/skills.rs`
- Create: `meos/crates/loopforge-cli/src/dispatch/skills/listing.rs`
- Create: `meos/crates/loopforge-cli/src/dispatch/skills/doctor.rs`
- Create: `meos/crates/loopforge-cli/src/dispatch/skills/run.rs`

**Steps:**
1. Move list/show printing helpers into `listing.rs`.
2. Move doctor report rendering and exit gating into `doctor.rs`.
3. Move runtime skill execution flow into `run.rs`.
4. Keep `dispatch/skills.rs` as a thin matcher only.

### Task 2: Verify no behavior change

**Files:**
- Verify: `meos/crates/loopforge-cli/src/dispatch/skills*.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p loopforge-cli --bin loopforge -- --nocapture`.
3. Run `cargo test --workspace --locked`.

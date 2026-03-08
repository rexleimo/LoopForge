# Onboard Flow Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Mechanically decompose `crates/loopforge-cli/src/onboard.rs` so the top-level file becomes a thin orchestrator with zero behavior change.

**Architecture:** Reuse the existing onboarding helper modules and split the remaining large `run` flow into focused helpers for bootstrap, preflight validation/doctor gating, and first-agent execution. Preserve stdout/stderr messages, exit behavior, and generated report content exactly.

**Tech Stack:** Rust, existing `loopforge-cli` onboard helpers, `rexos` runtime stack.

---

### Task 1: Extract bootstrap and preflight helpers

**Files:**
- Update: `meos/crates/loopforge-cli/src/onboard.rs`
- Create: `meos/crates/loopforge-cli/src/onboard/bootstrap.rs`
- Create: `meos/crates/loopforge-cli/src/onboard/preflight.rs`
- Create: `meos/crates/loopforge-cli/src/onboard/flow_types.rs`

**Steps:**
1. Move path/workspace/bootstrap setup into `bootstrap.rs`.
2. Move config/doctor gating into `preflight.rs`.
3. Store shared flow state in a small internal struct instead of passing large argument lists.

### Task 2: Extract first-agent execution helper

**Files:**
- Update: `meos/crates/loopforge-cli/src/onboard.rs`
- Create: `meos/crates/loopforge-cli/src/onboard/agent_flow.rs`

**Steps:**
1. Move the first-agent run and artifact verification path into `agent_flow.rs`.
2. Preserve failure categorization, retry hints, and report emission behavior exactly.
3. Keep `onboard.rs` as orchestration only.

### Task 3: Verify no behavior change

**Files:**
- Verify: `meos/crates/loopforge-cli/src/onboard*.rs`

**Steps:**
1. Run `cargo fmt --all`.
2. Run `cargo test -p loopforge-cli --bin loopforge onboard::tests -- --nocapture`.
3. Run `cargo test -p loopforge-cli --bin loopforge`.
4. Run `cargo test --workspace --locked`.

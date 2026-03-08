# Final OKR Sweep Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Finish the remaining bounded readability hotspots and public-doc polish tasks with zero runtime behavior change.

**Architecture:** Keep existing module APIs and behavior stable while splitting the remaining medium-sized runtime and CLI files into concern-focused submodules. Preserve public docs as user-facing guidance only, and keep maintainer-only detail in internal docs.

**Tech Stack:** Rust workspace crates (`rexos-runtime`, `loopforge-cli`), MkDocs docs-site, existing test suite.

---

### Task 1: Finish remaining runtime hotspot splits

**Files:**
- Update: `meos/crates/rexos-runtime/src/session_runner/tool_dispatch.rs`
- Create: `meos/crates/rexos-runtime/src/session_runner/tool_dispatch/*.rs`
- Update: `meos/crates/rexos-runtime/src/workflow/execution.rs`
- Create: `meos/crates/rexos-runtime/src/workflow/execution/*.rs`
- Update: `meos/crates/rexos-runtime/src/outbox.rs`
- Create: `meos/crates/rexos-runtime/src/outbox/*.rs`
- Update: `meos/crates/rexos-runtime/src/leak_guard/detect.rs`
- Create: `meos/crates/rexos-runtime/src/leak_guard/detect/*.rs`
- Update: `meos/crates/rexos-runtime/src/tool_calls/parse.rs`
- Create: `meos/crates/rexos-runtime/src/tool_calls/parse/*.rs`

**Steps:**
1. Split dispatch, workflow, outbox, leak-guard detection, and tool-call parsing into smaller concern-focused helpers.
2. Keep exported function signatures unchanged so callers and tests stay stable.
3. Avoid behavior changes; only move logic and reduce file concentration.

### Task 2: Finish remaining CLI hotspot splits and warning cleanup

**Files:**
- Update: `meos/crates/loopforge-cli/src/doctor/actions.rs`
- Create: `meos/crates/loopforge-cli/src/doctor/actions/*.rs`
- Update: `meos/crates/loopforge-cli/src/doctor/probes/config/runtime.rs`
- Create: `meos/crates/loopforge-cli/src/doctor/probes/config/runtime/*.rs`
- Update: `meos/crates/loopforge-cli/src/onboard.rs`

**Steps:**
1. Split doctor summary/next-actions logic and runtime probe logic into focused submodules.
2. Preserve all existing doctor outputs and test expectations.
3. Remove current `onboard` unused-import warnings by tightening test-only re-exports.

### Task 3: Finish remaining public docs polish sweep

**Files:**
- Update: `meos/docs-site/how-to/*.md`
- Update: `meos/docs-site/tutorials/*.md`
- Update: `meos/docs-site/zh-CN/how-to/*.md`
- Update: `meos/docs-site/zh-CN/tutorials/*.md`

**Steps:**
1. Add clearer user-facing framing to remaining how-to/tutorial pages.
2. Standardize around “when to use”, “recommended path”, and “how to verify”.
3. Keep competitor analysis and maintainer-only details out of public docs.

### Task 4: Run full verification

**Files:**
- Verify: `meos`

**Steps:**
1. Run targeted crate tests while refactoring if needed.
2. Run `cargo fmt --all --check`.
3. Run `python3 -m mkdocs build --strict`.
4. Run `cargo test --workspace --locked`.

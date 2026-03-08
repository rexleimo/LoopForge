# Doctor Hotspots Decomposition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce `crates/loopforge-cli/src/doctor.rs` into a thinner module boundary by moving its large embedded test block into a dedicated test file.

**Architecture:** Keep `doctor.rs` as the module root that re-exports types and the `run_doctor` entrypoint. Move only the `#[cfg(test)]` code into `crates/loopforge-cli/src/doctor/tests.rs` so runtime behavior and public module paths stay unchanged.

**Tech Stack:** Rust workspace, `cargo test`, targeted `rustfmt`.

## Task 1: Move doctor tests out of the root file
1. Replace the inline `#[cfg(test)] mod tests` block in `crates/loopforge-cli/src/doctor.rs` with `#[cfg(test)] mod tests;`.
2. Create `crates/loopforge-cli/src/doctor/tests.rs` with the exact existing tests.
3. Keep imports local to the new test file.

## Task 2: Verify behavior
1. Run `cargo test -p loopforge-cli --bin loopforge doctor::tests -- --nocapture`.
2. Run `cargo test -p loopforge-cli --bin loopforge`.
3. Run `cargo test --workspace --locked`.
4. Summarize file-size reduction and exact files touched.

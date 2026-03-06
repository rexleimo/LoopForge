# LoopForge Hard Cutover Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Hard-cut user-facing branding and runtime entrypoints from `LoopForge` to `LoopForge` without keeping backward-compatible public surfaces.

**Architecture:** Update core path/env/skill-discovery behavior first so runtime defaults align with the new product name, then sweep CLI/docs/scripts to match. Keep internal crate names intact unless they leak into user-facing output.

**Tech Stack:** Rust workspace, Clap CLI, Python scripts/tests, Bash scripts, MkDocs.

---

### Task 1: Lock runtime cutover behavior with failing tests

**Files:**
- Modify: `crates/rexos-kernel/src/paths.rs`
- Modify: `crates/loopforge-cli/src/main.rs`
- Modify: `crates/rexos/tests/harness_init.rs`
- Modify: `crates/rexos-skills/tests/loader_discovery.rs`
- Modify: `scripts/tests/test_provider_health_report.py`

**Step 1: Add failing path tests**
- Assert `RexosPaths::discover()` uses `~/.loopforge`.
- Assert default DB filename is `loopforge.db`.
- Assert only `.loopforge/skills` is used for workspace skill discovery.

**Step 2: Add failing UX tests**
- Assert CLI about/help text no longer mentions `LoopForge` or `~/.loopforge`.
- Assert harness init creates `loopforge-progress.md` and uses a `loopforge` commit subject.
- Assert provider health report reads `LOOPFORGE_*` env names.

**Step 3: Run targeted tests and confirm failures**
- `cargo test -p rexos-kernel --lib --locked`
- `cargo test -p loopforge-cli cli_primary_name_is_loopforge --locked`
- `cargo test -p rexos --test harness_init --locked`
- `cargo test -p rexos-skills --test loader_discovery --locked`
- `python3 -m unittest scripts.tests.test_provider_health_report`

### Task 2: Implement runtime hard cutover

**Files:**
- Modify: `crates/rexos-kernel/src/paths.rs`
- Modify: `crates/rexos-skills/src/loader.rs`
- Modify: `crates/rexos-harness/src/lib.rs`
- Modify: `crates/rexos-runtime/src/lib.rs`
- Modify: `crates/rexos-tools/src/lib.rs`
- Modify: `crates/loopforge-cli/src/main.rs`
- Modify: `crates/loopforge-cli/src/doctor.rs`
- Modify: `scripts/provider_health_report.py`
- Modify: `scripts/browser_sandbox_up.sh`
- Modify: `docker/sandbox-browser/compose.yml`
- Modify: `docker/sandbox-browser/entrypoint.sh`
- Modify: `docker/sandbox-browser/README.md`

**Step 1: Switch defaults**
- Change base dir to `~/.loopforge`.
- Change workspace artifact defaults to `.loopforge/...`.
- Change harness progress artifact to `loopforge-progress.md`.
- Change default DB/config/help text to `loopforge` naming.

**Step 2: Remove legacy compatibility lookups**
- Remove `.loopforge/skills` workspace discovery fallback.
- Replace `LOOPFORGE_*` public env names with `LOOPFORGE_*` where users configure behavior.

**Step 3: Update targeted tests until green**
- Re-run each targeted test from Task 1.

### Task 3: Sweep docs and examples to LoopForge-only wording

**Files:**
- Modify: `README.md`
- Modify: `README.zh-CN.md`
- Modify: `mkdocs.yml`
- Modify: `docs-site/**`
- Modify: `scripts/browser_sandbox_up.sh`
- Modify: `docker/sandbox-browser/README.md`

**Step 1: Replace public copy**
- Remove `` / `` wording.
- Replace `~/.loopforge`, `.loopforge`, `LOOPFORGE_*`, and `loopforge-progress.md` in docs/examples.

**Step 2: Keep internal-only names out of user guidance**
- Avoid changing crate/package names unless directly shown to users.

**Step 3: Run doc verification**
- `python -m mkdocs build --strict`
- Search for leftovers with `rg '\bLoopForge\b|\.loopforge|LOOPFORGE_'`

### Task 4: Run broader verification

**Files:**
- No new files

**Step 1: Run Python checks**
- `python3 -m unittest scripts.tests.test_provider_health_report`

**Step 2: Run focused Rust checks**
- `cargo test -p rexos-kernel --lib --locked`
- `cargo test -p rexos-skills --test loader_discovery --locked`
- `cargo test -p loopforge-cli --locked`
- `cargo test -p rexos --locked`

**Step 3: Run docs build**
- `test -d .venv-docs && . .venv-docs/bin/activate && python -m mkdocs build --strict`

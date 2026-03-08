# DevX Guardrails Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add repo-local guardrails and one-command verification for fresh clones (AGENTS.md + Makefile + CI fmt gate).

**Architecture:** Keep changes purely additive and low-risk: add small top-level docs and a `Makefile`, and extend CI with a dedicated `fmt` job. No runtime/CLI behavior changes.

**Tech Stack:** Rust (cargo), GitHub Actions, MkDocs (Material + i18n), Make.

---

### Task 1: Add repo-local agent/maintainer guardrails

**Files:**
- Create: `AGENTS.md`

**Step 1: Write the file**

Create `AGENTS.md` at repo root with:
- public docs boundary rules (`docs-site/` only)
- internal-only docs index link (`docs/internal/index.md`)
- required verification commands (`make check` or raw cargo/mkdocs commands)
- version/changelog rule reminder (`Cargo.toml` + `CHANGELOG.md` together when needed)

**Step 2: Verify it does not affect docs publishing**

Run: `python3 -m mkdocs build --strict`
Expected: exit 0 (MkDocs should ignore `AGENTS.md` by default since `docs_dir: docs-site`).

**Step 3: Commit**

Run:
```bash
git add AGENTS.md
git commit -m "docs: add agent guardrails"
```

---

### Task 2: Add a `Makefile` for one-command verification

**Files:**
- Create: `Makefile`

**Step 1: Add targets**

Include at minimum:
- `help`
- `fmt` / `fmt-check`
- `test`
- `docs`
- `docs-venv` (install `requirements-docs.txt` into `.venv-docs/`)
- `check` (fmt-check + test + docs)

**Step 2: Run a smoke check**

Run: `make help`
Expected: prints a short list of targets.

Run: `make fmt-check`
Expected: exit 0.

**Step 3: Commit**

Run:
```bash
git add Makefile
git commit -m "chore: add make targets for local checks"
```

---

### Task 3: Add CI formatting gate

**Files:**
- Modify: `.github/workflows/ci.yml`

**Step 1: Add a `fmt` job**

Add a job that:
- checks out code
- installs Rust toolchain
- restores rust cache
- runs `cargo fmt --all --check`

**Step 2: Run scripts workflow unit tests (optional but preferred)**

Run: `python3 -m unittest scripts.tests.test_ci_workflows`
Expected: pass.

**Step 3: Commit**

Run:
```bash
git add .github/workflows/ci.yml
git commit -m "ci: add cargo fmt check"
```

---

### Task 4: Verification + merge

**Files:**
- (no new files; verify whole tree)

**Step 1: Run full local verification**

Run: `make check`
Expected: exit 0.

**Step 2: Push and merge**

If working on a branch:
```bash
git push -u origin <branch>
```

Then merge to `main` using the project’s normal workflow (fast-forward or PR).

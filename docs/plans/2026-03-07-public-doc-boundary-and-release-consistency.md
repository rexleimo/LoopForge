# Public Docs Boundary and Release Consistency Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove competitor-analysis pages from the public LoopForge docs site, keep them as internal docs, and enforce public-doc and version/tag consistency rules in release preflight.

**Architecture:** Public-facing content remains under `docs-site/` and is built by MkDocs. Internal-only analysis moves to `docs/internal/competitive/`, which is not wired into MkDocs. Existing `loopforge release check` grows two lightweight checks: forbidden competitor terms in public docs and HEAD semver tag consistency.

**Tech Stack:** Rust CLI (`clap`, stdlib), MkDocs Material docs, Markdown content, existing release-check test module in `crates/loopforge-cli/src/main.rs`.

---

### Task 1: Move competitor-analysis docs out of public site

**Files:**
- Create: `docs/internal/competitive/loopforge-next-iteration-openfang-openclaw.en.md`
- Create: `docs/internal/competitive/loopforge-next-iteration-openfang-openclaw.zh-CN.md`
- Create: `docs/internal/competitive/loopforge-vs-openfang-openclaw.en.md`
- Create: `docs/internal/competitive/loopforge-vs-openfang-openclaw.zh-CN.md`
- Delete: `docs-site/blog/loopforge-next-iteration-openfang-openclaw.md`
- Delete: `docs-site/blog/loopforge-vs-openfang-openclaw.md`
- Delete: `docs-site/zh-CN/blog/loopforge-next-iteration-openfang-openclaw.md`
- Delete: `docs-site/zh-CN/blog/loopforge-vs-openfang-openclaw.md`

**Step 1:** Copy current public competitor-analysis pages into internal-only Markdown files.

**Step 2:** Remove the public copies from `docs-site/` so MkDocs cannot generate them.

**Step 3:** Keep filenames descriptive enough for internal discovery.

### Task 2: Remove public references and positioning leakage

**Files:**
- Modify: `mkdocs.yml`
- Modify: `docs-site/blog/index.md`
- Modify: `docs-site/zh-CN/blog/index.md`
- Modify: `docs-site/index.md`
- Modify: `docs-site/zh-CN/index.md`

**Step 1:** Remove competitor blog entries from the Blog nav in `mkdocs.yml`.

**Step 2:** Remove competitor reading-order links from EN/ZH blog index pages.

**Step 3:** Replace homepage hero comparison copy with neutral LoopForge positioning copy.

**Step 4:** Run `rg -n "openfang|openclaw|OpenFang|OpenClaw" docs-site mkdocs.yml` and expect no public hits.

### Task 3: Add failing tests for release-policy helpers

**Files:**
- Modify: `crates/loopforge-cli/src/main.rs`

**Step 1:** Add a failing test that a public docs scan reports forbidden competitor terms when they appear in `docs-site/` or `mkdocs.yml`.

**Step 2:** Add a failing test that HEAD semver tags fail release-check consistency when they do not match the requested tag.

**Step 3:** Run targeted tests and confirm they fail for the expected missing behavior.

### Task 4: Implement minimal release-check enforcement

**Files:**
- Modify: `crates/loopforge-cli/src/main.rs`

**Step 1:** Add a helper that scans public-doc roots for forbidden competitor-analysis terms.

**Step 2:** Add a helper that parses exact semver tags on HEAD from git output and evaluates consistency with the requested release tag.

**Step 3:** Wire both checks into `run_release_check` with clear `ReleaseCheckItem` messages.

**Step 4:** Re-run the targeted tests and confirm they pass.

### Task 5: Document the rules and verify end to end

**Files:**
- Modify: `docs/versioning-and-release.md`
- Modify: `CHANGELOG.md`

**Step 1:** Add internal rule text: public docs must not contain competitor-analysis content; internal strategy docs stay under `docs/internal/`.

**Step 2:** Add version/tag consistency rule text for `Cargo.toml`, `CHANGELOG.md`, and Git tag alignment.

**Step 3:** Add an `Unreleased` changelog note for the internal-doc boundary and release-check enforcement.

**Step 4:** Run:
- `cargo test -p loopforge-cli --locked`
- `python3 -m mkdocs build --strict`
- `cargo run -p loopforge-cli -- release check --tag v1.1.0`

**Step 5:** Review `git diff --stat` and ensure only intended files changed.


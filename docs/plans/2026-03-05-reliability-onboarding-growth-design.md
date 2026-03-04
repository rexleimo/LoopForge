# Reliability + Onboarding + Growth + Provider QA Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver four upgrades together: stabilize browser smoke tests, add one-command onboarding verification, build a provider quality reporting pipeline, and expand docs/blog growth assets.

**Architecture:** Keep runtime changes minimal and additive. Introduce one new stable browser smoke test and relax brittle assumptions in Baidu smoke. Add a CLI onboarding command that composes existing `init/config/doctor/agent` flow. Add a Python provider report script + nightly workflow + docs. Expand docs/blog with beginner-first guidance and operating cadence.

**Tech Stack:** Rust (`clap`, existing RexOS runtime), Python `unittest`, GitHub Actions, MkDocs.

---

### Task 1: Stabilize Real-World Browser Smoke (Baidu)

**Files:**
- Modify: `crates/rexos/tests/browser_baidu_weather_smoke.rs`
- Test: `crates/rexos/tests/browser_baidu_weather_smoke.rs`

**Step 1: Write/adjust failing expectation**

- Replace hard weather-text assertion with robust criteria:
  - page URL is Baidu
  - page text is non-trivial length
  - summary is non-empty
- Keep artifacts (screenshot/page dump) unchanged.

**Step 2: Run test to verify current brittleness**

Run:
```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_baidu_weather_smoke -- --ignored --nocapture
```
Expected (before fix): intermittent FAIL on `weather` keyword or results selector timeout.

**Step 3: Implement minimal robust logic**

- Make `browser_wait_for` best-effort.
- Accept anti-bot/security pages as valid “browser reached target domain”.
- Keep weather extraction prompt and evidence printing.

**Step 4: Re-run smoke**

Run same command and verify no brittle keyword dependency remains.

**Step 5: Commit**

```bash
git add crates/rexos/tests/browser_baidu_weather_smoke.rs
git commit -m "test: stabilize baidu browser smoke assertions"
```

### Task 2: Add Stable Browser + Ollama Baseline Smoke

**Files:**
- Create: `crates/rexos/tests/browser_wikipedia_smoke.rs`
- Modify: `docs-site/how-to/browser-use-cases/smoke-test.md`
- Modify: `docs-site/zh-CN/how-to/browser-use-cases/smoke-test.md`

**Step 1: Add a new ignored stable smoke test**

- Navigate to `https://www.wikipedia.org`
- Read page text
- Take screenshot
- Ask Ollama for a 1-paragraph summary
- Assert non-empty summary

**Step 2: Run new smoke test (expected pass)**

```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_wikipedia_smoke -- --ignored --nocapture
```

**Step 3: Update docs to recommend baseline-first**

- Document “stable smoke first, scenario smoke second”.

**Step 4: Commit**

```bash
git add crates/rexos/tests/browser_wikipedia_smoke.rs docs-site/how-to/browser-use-cases/smoke-test.md docs-site/zh-CN/how-to/browser-use-cases/smoke-test.md
git commit -m "docs(test): add stable wikipedia browser smoke guidance"
```

### Task 3: Add One-Command Onboarding Verification (`rexos onboard`)

**Files:**
- Modify: `crates/rexos-cli/src/main.rs`
- Test: `crates/rexos-cli/src/main.rs` (CLI parse/unit tests section)
- Modify: `docs-site/tutorials/new-user-walkthrough.md`
- Modify: `docs-site/zh-CN/tutorials/new-user-walkthrough.md`

**Step 1: Add failing parser test**

Add CLI parse test for:
```text
rexos onboard --workspace rexos-onboard-demo
```

**Step 2: Run targeted test**

```bash
cargo test -p rexos-cli cli_parses_onboard_subcommand
```
Expected (before implementation): FAIL.

**Step 3: Implement subcommand**

- Add `Command::Onboard { workspace, prompt, skip_agent }`
- Flow:
  1. Ensure `~/.rexos` + default config + db
  2. Validate config
  3. Run doctor (text output)
  4. Optionally run first agent prompt in workspace
  5. Print `session_id` when agent step runs

**Step 4: Re-run CLI tests**

```bash
cargo test -p rexos-cli
```

**Step 5: Update walkthrough docs**

- Add one-command onboarding path and expected outputs.

**Step 6: Commit**

```bash
git add crates/rexos-cli/src/main.rs docs-site/tutorials/new-user-walkthrough.md docs-site/zh-CN/tutorials/new-user-walkthrough.md
git commit -m "feat(cli): add rexos onboard one-command verification flow"
```

### Task 4: Provider Quality Report Script + Nightly Workflow

**Files:**
- Create: `scripts/provider_health_report.py`
- Create: `scripts/tests/test_provider_health_report.py`
- Modify: `.github/workflows/ci.yml`
- Create: `.github/workflows/provider-nightly.yml`
- Modify: `docs-site/how-to/providers.md`
- Modify: `docs-site/zh-CN/how-to/providers.md`

**Step 1: Add failing unit tests for report script**

- Validate model of report entries.
- Validate command-generation logic from env variables.

**Step 2: Implement script**

- Collect timestamp + host metadata.
- Detect available provider env vars.
- Build runnable smoke commands:
  - Ollama smoke
  - optional GLM/MiniMax/NVIDIA when env vars exist
- Output markdown + json report under `.tmp/provider-health/`.

**Step 3: Wire CI script tests**

- Add `scripts.tests.test_provider_health_report` to `scripts-tests` job.

**Step 4: Add nightly workflow**

- `schedule` + `workflow_dispatch`
- Run report script and upload artifacts
- Do not fail entire workflow if optional providers are missing keys.

**Step 5: Update provider docs**

- Add section for running report script locally.
- Add interpretation guide for pass/fail/skipped.

**Step 6: Commit**

```bash
git add scripts/provider_health_report.py scripts/tests/test_provider_health_report.py .github/workflows/ci.yml .github/workflows/provider-nightly.yml docs-site/how-to/providers.md docs-site/zh-CN/how-to/providers.md
git commit -m "feat(qa): add provider health report and nightly workflow"
```

### Task 5: Docs Growth Cadence + Blog Pipeline

**Files:**
- Create: `docs-site/blog/editorial-calendar.md`
- Create: `docs-site/zh-CN/blog/editorial-calendar.md`
- Modify: `docs-site/blog/index.md`
- Modify: `docs-site/zh-CN/blog/index.md`
- Modify: `mkdocs.yml`

**Step 1: Add cadence docs**

- Weekly publishing template:
  - problem
  - copy/paste commands
  - verification
  - pitfalls
  - migration tips

**Step 2: Surface in blog nav**

- Add “Editorial Calendar” in EN + zh-CN nav.

**Step 3: Commit**

```bash
git add docs-site/blog/editorial-calendar.md docs-site/zh-CN/blog/editorial-calendar.md docs-site/blog/index.md docs-site/zh-CN/blog/index.md mkdocs.yml
git commit -m "docs(blog): add editorial cadence and publishing template"
```

### Task 6: Full Verification

**Files:**
- N/A (verification only)

**Step 1: Rust tests**

```bash
cargo test --workspace --locked
```

**Step 2: Scripts tests**

```bash
python3 -m unittest \
  scripts.tests.test_ci_workflows \
  scripts.tests.test_verify_version_changelog \
  scripts.tests.test_verify_release_consistency \
  scripts.tests.test_provider_health_report
```

**Step 3: Docs build**

```bash
python3 -m mkdocs build --strict
```

**Step 4: Real Ollama smoke checks**

```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test ollama_smoke -- --ignored --nocapture
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_wikipedia_smoke -- --ignored --nocapture
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_baidu_weather_smoke -- --ignored --nocapture
```

**Step 5: CLI onboarding verification**

```bash
rexos onboard --workspace .tmp/onboard-demo --prompt "Create hello.txt with the word hi"
```

**Step 6: Final commit (if needed)**

```bash
git add -A
git commit -m "chore: finalize reliability and onboarding upgrades"
```

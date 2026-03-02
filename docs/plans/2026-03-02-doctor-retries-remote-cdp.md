# Doctor + LLM Retries + Remote CDP Hardening (Implementation Plan)

> **For Claude:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task.

**Goal:** Improve RexOS “new user” experience and runtime reliability by adding a `rexos doctor` command, adding safe HTTP retry/backoff for LLM calls, and hardening remote CDP attachment defaults.

**Architecture:** Keep the public tool/CLI surface stable. Add `doctor` as a CLI subcommand that runs non-destructive checks (paths/config/db/providers/browser/git/docker) and prints actionable guidance. Add bounded retries for transient HTTP errors (429/5xx) in LLM clients. For `REXOS_BROWSER_CDP_HTTP`, default to loopback-only and require an explicit opt-in env var to attach to non-loopback CDP endpoints.

**Tech Stack:** Rust (clap, reqwest, tokio), MkDocs Material + mkdocs-static-i18n.

---

## Task 1: Add `rexos doctor` CLI command

**Files:**
- Modify: `crates/rexos-cli/src/main.rs`
- Create: `crates/rexos-cli/src/doctor.rs`
- Modify: `crates/rexos-cli/Cargo.toml`
- Test: `crates/rexos-cli/src/doctor.rs` (unit tests with local Axum server)

**Step 1: Define the CLI surface**
- Add `Command::Doctor` with flags:
  - `--json` (machine readable)
  - `--timeout-ms` (network probe timeout)
  - `--strict` (exit non-zero on warnings)

**Step 2: Implement checks (non-destructive)**
- Paths:
  - Print `~/.rexos` base dir, config path, db path.
  - Warn if config/db missing (suggest `rexos init`).
- Providers:
  - Parse `config.toml`.
  - Verify router provider names exist.
  - For providers with `api_key_env`, warn if env var not set.
  - For local `ollama` (openai-compatible, no key), probe `GET {base_url}/models`.
- Browser:
  - If `REXOS_BROWSER_CDP_HTTP` is set, probe `GET {cdp}/json/version`.
  - Otherwise, try a best-effort Chromium discovery (PATH + common locations) and warn if missing.
- Tools:
  - `git --version` (warn if missing; required for harness).
  - `docker --version` (info/warn; only needed for sandbox GUI).

**Step 3: Add unit tests**
- Use a local Axum server for `/models` and `/json/version` probes.
- Assert JSON output includes expected statuses.

**Verification**
- Run: `cargo test -p rexos-cli`

---

## Task 2: Add LLM HTTP retries for transient errors

**Files:**
- Modify: `crates/rexos-llm/Cargo.toml`
- Modify: `crates/rexos-llm/src/openai_compat.rs`
- (Optional) Modify: `crates/rexos-llm/src/zhipu.rs`, `crates/rexos-llm/src/minimax.rs`
- Test: `crates/rexos/tests/llm_openai_compat.rs`

**Step 1: Write a failing test**
- Add a test server that returns 503 once, then 200 with a valid response.
- Assert `OpenAiCompatibleClient::chat_completions` succeeds.

**Step 2: Implement bounded retry**
- Retry on 429/500/502/503/504 up to `REXOS_LLM_RETRY_MAX` (default 2).
- Add exponential backoff with jitter:
  - base 250ms, cap 2s.

**Verification**
- Run: `cargo test -p rexos --test llm_openai_compat`

---

## Task 3: Harden remote CDP attachment defaults

**Files:**
- Modify: `crates/rexos-tools/src/browser_cdp.rs`
- Test: `crates/rexos-tools/src/browser_cdp.rs` (unit tests)
- Docs: `docs-site/how-to/browser-automation.md` + `docs-site/zh-CN/how-to/browser-automation.md`

**Step 1: Add guardrails**
- If `REXOS_BROWSER_CDP_HTTP` host is NOT loopback (`127.0.0.1`/`localhost`/`::1`):
  - Fail with a clear message unless `REXOS_BROWSER_CDP_ALLOW_REMOTE=1`.

**Step 2: Tests**
- Unit test the host validation (no actual browser required).

**Verification**
- Run: `cargo test -p rexos-tools`
- Run: `python3 -m mkdocs build --strict`

---

## Task 4: Docs + examples polish

**Files:**
- Modify: `docs-site/examples/sanity-check.md`
- Modify: `docs-site/zh-CN/examples/sanity-check.md`
- Modify: `docs-site/how-to/troubleshooting.md`
- Modify: `docs-site/zh-CN/how-to/troubleshooting.md`

**Steps**
- Add `rexos doctor` as the first step for newcomers.
- Add a “Browser sandbox quick verify” snippet referencing the GUI smoke check.

**Verification**
- Run: `python3 -m mkdocs build --strict`


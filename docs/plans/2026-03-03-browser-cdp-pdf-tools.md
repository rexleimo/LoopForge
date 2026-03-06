# Browser CDP Enhancements + PDF Tool Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task.

**Goal:** Make the CDP browser backend more reliable in proxied environments and add a first-class PDF text extraction tool (`pdf`) for agent workflows.

**Architecture:** Keep the existing `browser_*` tool surface stable. Improve CDP by using a dedicated `reqwest::Client` with proxies disabled for **loopback CDP HTTP endpoints**. Add a small env-configurable switch for how we select a CDP target/tab. Add a `pdf` tool to `rexos-tools` that extracts text from workspace-relative PDFs (size + sandbox checks, page/char limits, deterministic JSON output).

**Tech Stack:** Rust (`tokio`, `reqwest`, `tokio-tungstenite`), `pdf-extract` (text extraction), MkDocs Material + `mkdocs-static-i18n`.

---

## Task 1: CDP loopback proxy bypass (reliability)

**Why:** When `HTTP_PROXY/HTTPS_PROXY` are set, some environments accidentally route `http://127.0.0.1:<port>/json/*` through a proxy. That breaks local CDP sessions.

**Files:**
- Modify: `crates/rexos-tools/src/browser_cdp.rs`
- Test: `crates/rexos-tools/src/browser_cdp.rs`

**Step 1: Write failing test**

- Add a `#[tokio::test]` that:
  - starts a local HTTP server exposing `GET /json/new` returning a valid `webSocketDebuggerUrl`
  - sets `HTTP_PROXY=http://127.0.0.1:1`, `HTTPS_PROXY=http://127.0.0.1:1`, and clears `NO_PROXY`
  - calls `find_or_create_page_ws()` using the shared client
  - expects success (it must still reach loopback despite proxy env)

Run: `cargo test -p rexos-tools find_or_create_page_ws_bypasses_proxy_for_loopback`
Expected: FAIL before implementation (cannot reach loopback due to proxy).

**Step 2: Implement**

- In `find_or_create_page_ws()`, when `base.host_str()` is loopback (`127.0.0.1/localhost/::1`), create a small one-off `reqwest::Client` via `Client::builder().no_proxy()` and use it for `/json/new` + `/json/list`.

**Step 3: Verify**

Run: `cargo test -p rexos-tools find_or_create_page_ws_bypasses_proxy_for_loopback`
Expected: PASS.

---

## Task 2: CDP tab selection mode (remote compatibility)

**Why:** Some remote CDP setups do not allow `/json/new` (or users want to reuse an existing tab instead of creating new ones).

**Files:**
- Modify: `crates/rexos-tools/src/browser_cdp.rs`
- Modify: `docs-site/reference/tools.md`
- Modify: `docs-site/zh-CN/reference/tools.md`

**Step 1: Write failing test**

- Add an env `LOOPFORGE_BROWSER_CDP_TAB_MODE=reuse|new` (default `new`).
- Add a `#[tokio::test]` that sets `LOOPFORGE_BROWSER_CDP_TAB_MODE=reuse` and runs against a server that:
  - fails `/json/new`
  - succeeds `/json/list` and increments a counter if `/json/new` is called
- Expect: the counter is `0` (reuse mode must not call `/json/new`).

Run: `cargo test -p rexos-tools cdp_tab_mode_reuse_skips_json_new`
Expected: FAIL before implementation.

**Step 2: Implement**

- Parse `LOOPFORGE_BROWSER_CDP_TAB_MODE` once per call (cheap):
  - `new` (default): try `/json/new` first, then fall back to `/json/list`
  - `reuse`: skip `/json/new` entirely; only use `/json/list`

**Step 3: Verify**

Run: `cargo test -p rexos-tools cdp_tab_mode_reuse_skips_json_new`
Expected: PASS.

**Step 4: Document**

- Mention `LOOPFORGE_BROWSER_CDP_TAB_MODE` in tools reference (en + zh-CN).

---

## Task 3: Add `pdf` tool definition + tool dispatch

**Files:**
- Modify: `crates/rexos-tools/Cargo.toml`
- Modify: `crates/rexos-tools/src/lib.rs`
- Test: `crates/rexos-tools/src/lib.rs`

**Tool contract (MVP):**

- Tool name: `pdf` (compat alias: `pdf_extract`)
- Args:
  - `path` (workspace-relative)
  - optional `max_pages` (default 10, hard cap 50)
  - optional `max_chars` (default 12000, hard cap 50000)
- Output JSON:
  - `path`
  - `text` (truncated to `max_chars`)
  - `truncated` (bool)
  - `bytes`
  - optional `pages_extracted` (u64)

**Step 1: Write failing tests**

- Add `tool_definitions_include_pdf` to ensure `pdf` appears in `Toolset::definitions()`.
- Add `pdf_extracts_text_from_fixture` using a tiny embedded PDF fixture (base64) written to the workspace.

Run: `cargo test -p rexos-tools tool_definitions_include_pdf pdf_extracts_text_from_fixture`
Expected: FAIL (tool missing).

**Step 2: Add dependency**

- Add `pdf-extract = "0.10"` to `crates/rexos-tools/Cargo.toml`.

**Step 3: Implement minimal tool**

- Implement `Toolset::pdf_extract()` that:
  - resolves workspace-relative `path`
  - enforces a max file size (ex: 20 MiB hard limit)
  - calls `pdf_extract::extract_text` (best-effort)
  - normalizes whitespace lightly and truncates by `max_chars`

Wire it into:
- `Toolset::definitions()` (add `pdf` def)
- `Toolset::call()` (match `"pdf" | "pdf_extract"`)

**Step 4: Verify**

Run: `cargo test -p rexos-tools tool_definitions_include_pdf pdf_extracts_text_from_fixture`
Expected: PASS.

---

## Task 4: Docs + copy/paste case task

**Files:**
- Modify: `docs-site/reference/tools.md`
- Modify: `docs-site/zh-CN/reference/tools.md`
- Create: `docs-site/examples/case-tasks/pdf-summarize.md`
- Create: `docs-site/zh-CN/examples/case-tasks/pdf-summarize.md`
- Modify: `mkdocs.yml` (add nav entry for PDF case task)

**Step 1: Add tools reference section**

- Add a short `## \`pdf\`` section showing args + output example.

**Step 2: Add case task page**

- Provide a runnable prompt for: “extract PDF text with `pdf`, then summarize into `notes/<name>.md`”.
- Include both Bash + PowerShell examples.

**Step 3: Verify docs build**

Run:
- `python3 -m pip install -r requirements-docs.txt`
- `python3 -m mkdocs build --strict`
Expected: PASS.

---

## Task 5: Verify, commit, merge

Run:
- `cargo test`
- `python3 -m mkdocs build --strict`

Then:
- commit in feature branch
- merge back to `main`
- push `main` to GitHub


# LoopForge Security Hardening Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add encrypted secret resolution, explicit outbound allowlists, and leak-guarded tool output to LoopForge without changing its local-first software-delivery focus.

**Architecture:** Extend `rexos-kernel` with security config and secret resolution, extend `rexos-tools` with a reusable egress policy validator, and extend `rexos-runtime` with leak scanning and sanitized audit persistence. Surface posture through `loopforge doctor` and keep the first rollout backward-compatible via env fallback.

**Tech Stack:** Rust workspace (`rexos-kernel`, `rexos-tools`, `rexos-runtime`, `loopforge-cli`), `serde`, `toml`, existing `reqwest`/`tokio` test infrastructure, existing audit records and CLI doctor output.

---

### Task 1: Add security config types

**Files:**
- Create: `crates/rexos-kernel/src/security.rs`
- Modify: `crates/rexos-kernel/src/lib.rs`
- Modify: `crates/rexos-kernel/src/config.rs`
- Test: `crates/rexos-kernel/src/config.rs`

**Step 1: Write the failing test**

Add config parsing tests that expect these new sections to deserialize:

```toml
[security.secrets]
mode = "env_first"

[security.leaks]
mode = "warn"

[[security.egress.rules]]
tool = "web_fetch"
host = "docs.rs"
path_prefix = "/"
methods = ["GET"]
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p rexos-kernel --locked`
Expected: parse or missing-type failure for `security` fields.

**Step 3: Write minimal implementation**

Add:
- `SecurityConfig`
- `SecretsConfig`
- `LeakGuardConfig`
- `EgressConfig`
- `EgressRule`
- `LeakMode` enum
- `SecretMode` enum

Keep defaults backward-compatible so existing configs still load.

**Step 4: Run test to verify it passes**

Run: `cargo test -p rexos-kernel --locked`
Expected: new config tests pass and existing config tests stay green.

**Step 5: Commit**

```bash
git add crates/rexos-kernel/src/security.rs crates/rexos-kernel/src/lib.rs crates/rexos-kernel/src/config.rs
git commit -m "feat: add security config model"
```

### Task 2: Add host-side secret resolution

**Files:**
- Create: `crates/rexos-kernel/src/secrets.rs`
- Modify: `crates/rexos-kernel/src/lib.rs`
- Modify: `crates/rexos-kernel/src/config.rs`
- Modify: `crates/rexos-llm/src/registry.rs`
- Modify: `crates/rexos-llm/src/anthropic.rs`
- Modify: `crates/rexos-llm/src/gemini.rs`
- Modify: `crates/rexos-llm/src/dashscope.rs`
- Modify: `crates/rexos-llm/src/zhipu.rs`
- Test: `crates/rexos/tests/llm_anthropic.rs`
- Test: `crates/rexos/tests/llm_gemini.rs`
- Test: `crates/rexos/tests/llm_dashscope.rs`
- Test: `crates/rexos/tests/llm_zhipu.rs`

**Step 1: Write the failing test**

Add resolver tests for:
- env lookup success
- blank env name returns `None`
- explicit provider key lookup still works through the resolver

**Step 2: Run test to verify it fails**

Run: `cargo test -p rexos --locked --test llm_anthropic --test llm_gemini --test llm_dashscope --test llm_zhipu`
Expected: compile failure or missing resolver behavior.

**Step 3: Write minimal implementation**

Create a small resolver API such as:

```rust
pub struct SecretResolver { /* config + paths */ }
impl SecretResolver {
    pub fn resolve_env(&self, env_name: &str) -> Option<String> { /* env_first */ }
    pub fn resolve_provider_api_key(&self, config: &RexosConfig, provider: &str) -> Option<String> { /* delegate */ }
}
```

Do not add keychain support yet. Keep the abstraction ready for that follow-up.

**Step 4: Run test to verify it passes**

Run the same targeted LLM tests again.
Expected: providers still authenticate through the new resolver path.

**Step 5: Commit**

```bash
git add crates/rexos-kernel/src/secrets.rs crates/rexos-kernel/src/lib.rs crates/rexos-kernel/src/config.rs crates/rexos-llm/src/registry.rs crates/rexos-llm/src/anthropic.rs crates/rexos-llm/src/gemini.rs crates/rexos-llm/src/dashscope.rs crates/rexos-llm/src/zhipu.rs crates/rexos/tests/llm_anthropic.rs crates/rexos/tests/llm_gemini.rs crates/rexos/tests/llm_dashscope.rs crates/rexos/tests/llm_zhipu.rs
git commit -m "refactor: route provider secrets through resolver"
```

### Task 3: Add reusable egress policy matching

**Files:**
- Create: `crates/rexos-tools/src/net/policy.rs`
- Modify: `crates/rexos-tools/src/net.rs`
- Modify: `crates/rexos-tools/src/net/tests.rs`
- Modify: `crates/rexos-kernel/src/security.rs`
- Test: `crates/rexos-tools/src/net/tests.rs`

**Step 1: Write the failing test**

Add policy tests for:
- exact host match
- path prefix match
- method mismatch deny
- empty rule set deny for protected mode
- public URL with matching rule allow

**Step 2: Run test to verify it fails**

Run: `cargo test -p rexos-tools --locked net::tests`
Expected: missing `policy` module or failed rule-matching assertions.

**Step 3: Write minimal implementation**

Add a validator API like:

```rust
pub(crate) fn egress_rule_allows(
    tool_name: &str,
    method: &str,
    url: &reqwest::Url,
    config: &SecurityConfig,
) -> anyhow::Result<()> { /* allow or deny with stable reason */ }
```

Keep IP-class filtering separate from rule matching so failures are understandable.

**Step 4: Run test to verify it passes**

Run: `cargo test -p rexos-tools --locked`
Expected: new network-policy tests pass with the existing IP tests.

**Step 5: Commit**

```bash
git add crates/rexos-tools/src/net/policy.rs crates/rexos-tools/src/net.rs crates/rexos-tools/src/net/tests.rs crates/rexos-kernel/src/security.rs
git commit -m "feat: add reusable egress policy validator"
```

### Task 4: Wire egress policy into networked tools

**Files:**
- Modify: `crates/rexos-tools/src/ops/web/remote.rs`
- Modify: `crates/rexos-tools/src/ops/web/fetch/mod.rs`
- Modify: `crates/rexos-tools/src/ops/web/a2a/send/mod.rs`
- Modify: `crates/rexos-tools/src/ops/web/a2a/discover/url.rs`
- Modify: `crates/rexos-tools/src/defs/browser/network/allow.rs`
- Modify: `crates/rexos-tools/src/tests/web/fetch.rs`
- Modify: `crates/rexos-tools/src/tests/web/a2a.rs`
- Modify: `crates/rexos-tools/src/tests/browser/policy/url.rs`
- Modify: `crates/rexos/tests/tools_web_fetch.rs`

**Step 1: Write the failing test**

Add integration coverage that:
- denies `web_fetch` to an unmatched public host when egress mode is enforced
- allows `web_fetch` to a matched host/path/method
- denies browser navigation when no matching rule exists
- keeps `allow_private = true` as an explicit override path during rollout

**Step 2: Run test to verify it fails**

Run: `cargo test -p rexos-tools --locked` and `cargo test -p rexos --locked --test tools_web_fetch`
Expected: network tools ignore policy until the new plumbing is added.

**Step 3: Write minimal implementation**

Thread the loaded `SecurityConfig` into the places that currently call:
- `ensure_remote_url_allowed(...)`
- `ensure_browser_url_allowed(...)`

Use stable denial messages that identify the blocked layer.

**Step 4: Run test to verify it passes**

Run the same commands again.
Expected: policy-driven allow/deny behavior is covered and old private-network tests still pass.

**Step 5: Commit**

```bash
git add crates/rexos-tools/src/ops/web/remote.rs crates/rexos-tools/src/ops/web/fetch/mod.rs crates/rexos-tools/src/ops/web/a2a/send/mod.rs crates/rexos-tools/src/ops/web/a2a/discover/url.rs crates/rexos-tools/src/defs/browser/network/allow.rs crates/rexos-tools/src/tests/web/fetch.rs crates/rexos-tools/src/tests/web/a2a.rs crates/rexos-tools/src/tests/browser/policy/url.rs crates/rexos/tests/tools_web_fetch.rs
git commit -m "feat: enforce egress policy across networked tools"
```

### Task 5: Add runtime leak guard

**Files:**
- Create: `crates/rexos-runtime/src/leak_guard.rs`
- Modify: `crates/rexos-runtime/src/lib.rs`
- Modify: `crates/rexos-runtime/src/records.rs`
- Modify: `crates/rexos/tests/runtime_controls.rs`
- Modify: `crates/rexos/tests/skills_audit_events.rs`

**Step 1: Write the failing test**

Add tests that expect:
- `warn` mode preserves output but marks the audit
- `redact` mode masks matched content
- `enforce` mode blocks persistence of raw leaked text

**Step 2: Run test to verify it fails**

Run: `cargo test -p rexos --locked --test runtime_controls --test skills_audit_events`
Expected: leak metadata and sanitization hooks do not exist yet.

**Step 3: Write minimal implementation**

Create a small leak-guard module with:
- match scanning for high-confidence token patterns
- `sanitize_output(mode, text)` helper
- audit metadata fields such as `leak_detected` and `sanitized`

Keep the first version simple and deterministic.

**Step 4: Run test to verify it passes**

Run the same targeted runtime tests.
Expected: audits store only sanitized output when configured.

**Step 5: Commit**

```bash
git add crates/rexos-runtime/src/leak_guard.rs crates/rexos-runtime/src/lib.rs crates/rexos-runtime/src/records.rs crates/rexos/tests/runtime_controls.rs crates/rexos/tests/skills_audit_events.rs
git commit -m "feat: add runtime leak guard"
```

### Task 6: Surface posture through doctor and docs

**Files:**
- Modify: `crates/loopforge-cli/src/doctor.rs`
- Modify: `crates/loopforge-cli/src/main.rs`
- Modify: `docs-site/explanation/security.md`
- Modify: `docs-site/reference/config.md`
- Create: `docs/internal/loopforge-network-security.md`
- Test: `crates/loopforge-cli/src/doctor.rs`

**Step 1: Write the failing test**

Add doctor tests that expect security posture output such as:
- `security.egress: configured|not configured`
- `security.leaks: off|warn|redact|enforce`
- `security.secrets: env_first`

**Step 2: Run test to verify it fails**

Run: `cargo test -p loopforge-cli --locked`
Expected: doctor output does not mention security posture yet.

**Step 3: Write minimal implementation**

Extend doctor reporting and document:
- how to configure egress rules
- how leak modes behave
- that provider credentials still support env fallback in the first rollout

Keep competitor names out of all public docs.

**Step 4: Run test to verify it passes**

Run: `cargo test -p loopforge-cli --locked`
Expected: CLI tests pass and new doctor strings are stable.

**Step 5: Commit**

```bash
git add crates/loopforge-cli/src/doctor.rs crates/loopforge-cli/src/main.rs docs-site/explanation/security.md docs-site/reference/config.md docs/internal/loopforge-network-security.md
git commit -m "docs: expose security posture and configuration"
```

### Task 7: Run full regression verification

**Files:**
- Modify: `CHANGELOG.md` (only if implementation actually lands in this batch)

**Step 1: Run focused workspace verification**

Run:

```bash
cargo test -p rexos-kernel --locked
cargo test -p rexos-tools --locked
cargo test -p loopforge-cli --locked
cargo test -p rexos --locked --test runtime_controls --test tools_web_fetch --test llm_anthropic --test llm_gemini --test llm_dashscope --test llm_zhipu
cargo fmt --all --check
```

Expected: all targeted suites pass.

**Step 2: Run full verification**

Run:

```bash
cargo test --workspace
cargo run -p loopforge-cli -- release check --tag v1.2.0
```

Expected: full workspace green, release preflight still passes.

**Step 3: Decide version impact**

If the batch remains backward-compatible and user-visible, treat it as a `minor` or `patch` according to the final scope. Update `Cargo.toml` and `CHANGELOG.md` in the same change set if a version bump is required.

**Step 4: Summarize rollout outcome**

Record:
- which tools are policy-enforced
- whether secrets are still env-only or partially sealed
- which leak modes shipped
- any intentionally deferred items such as OS keychain support

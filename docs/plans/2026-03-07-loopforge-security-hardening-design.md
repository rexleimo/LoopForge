# LoopForge Security Hardening Design

Date: 2026-03-07

## Goal

Lift LoopForge from approval-first safety to policy-first safety by adding three internal security primitives inspired by IronClaw:

1. encrypted secret resolution at the host boundary
2. explicit outbound egress policy for networked tools
3. leak scanning before sensitive output is displayed or persisted

## Recommendation

Use **Compose + Extend**.

Do not import IronClaw’s full architecture. Instead, extend the existing LoopForge workspace with narrowly scoped modules in `rexos-kernel`, `rexos-tools`, `rexos-runtime`, and `loopforge-cli`.

## Non-Goals

- no PostgreSQL / pgvector requirement
- no gateway/orchestrator/worker rewrite
- no full WASM sandbox in this slice
- no public docs that mention competitor names
- no breaking CLI changes for existing `env`-based provider credentials

## Design Overview

### 1. Kernel-owned security configuration

Add a new internal configuration surface under the root config:

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

Design rules:
- defaults must preserve current behavior for existing users
- `allow_private = true` continues to work as an explicit override during transition
- config parsing lives in `rexos-kernel`, not in individual tool implementations

### 2. Secret resolution boundary

Add a host-only secret resolver that answers the question: “what credential should this provider/tool use?”

Design rules:
- initial mode supports `env_first`
- follow-up mode can add system keychain sealing without forcing a migration
- tool/runtime code receives only the resolved value it needs for the current call
- doctor output and audit records never print secret values

Recommended module shape:
- `rexos-kernel::security` for config and policies
- `rexos-kernel::secrets` for resolving named credentials

### 3. Egress policy boundary

Replace the coarse `allow_private`-only posture with a reusable validator that can answer:
- which tool is making the request?
- which host is targeted?
- which path prefix is requested?
- which HTTP method is used?
- is the request public, private, or loopback?

Design rules:
- centralize policy matching under `rexos-tools/src/net/`
- keep URL normalization and IP checks in one place
- return denial reasons that are precise enough for operator debugging
- wire the same validator into `web_fetch`, A2A, and browser navigation

### 4. Leak guard boundary

Add a runtime output scanner that runs before:
- tool output is returned to the model/user
- tool audit records are persisted
- ACP or other delivery events capture sensitive strings

Modes:
- `off`: preserve current behavior
- `warn`: log and annotate audits
- `redact`: replace matched ranges before persistence/display
- `enforce`: fail the call with a stable error

Design rules:
- keep the first version regex/pattern based
- store only summary metadata in audits, not raw matched secrets
- integrate with existing audit records rather than inventing a parallel system

### 5. CLI / operator visibility

Extend `loopforge doctor` so maintainers can tell:
- whether security config is present
- whether egress rules are configured
- whether the runtime is still env-only for provider secrets
- whether leak guard is enabled and in which mode

This keeps security posture inspectable without adding a heavy UI surface.

## Data Flow

### Secret resolution

1. provider/tool requests a named credential
2. kernel secret resolver loads config
3. resolver checks sealed store (future) then env fallback
4. caller receives the value for in-memory use only
5. no audit log prints the raw secret

### Networked tool request

1. tool parser extracts destination URL + method + tool name
2. `EgressPolicy` evaluates rule match
3. loopback/private IP checks still run
4. request proceeds only if both network class and rule policy allow it

### Tool output

1. tool returns raw output
2. runtime leak guard scans output
3. runtime warns, redacts, or blocks based on config
4. sanitized output is what reaches audits and the caller

## Error Handling

- invalid security config should fail early in config parsing
- egress denials should report whether the miss was scheme, host, path, method, or private-network class
- leak-guard blocks should return stable user-facing errors and keep raw content out of persisted logs
- missing credentials should continue to surface as actionable provider errors, not opaque “security failure” messages

## Testing Strategy

### Unit tests

- config parsing defaults and new `security.*` blocks
- egress rule matching and precedence
- path normalization and encoded-separator rejection
- leak guard mode behavior: off / warn / redact / enforce
- secret resolver fallback order

### Integration tests

- `web_fetch` obeys allowlist rules
- browser navigation obeys allowlist rules
- A2A requests obey allowlist rules
- provider calls keep working with `env_first`
- tool audits keep sanitized values only when leak guard is active

### Regression tests

- existing private-network denial tests continue to pass
- existing approval policy tests continue to pass
- existing provider routing tests continue to pass

## Rollout Order

1. internal config + egress validator
2. wire validator into existing networked tools
3. add leak guard with audit integration
4. add secret resolver abstraction with env fallback
5. expose posture in `loopforge doctor`
6. document public-safe security guidance without mentioning IronClaw

## Success Criteria

- LoopForge gains a reusable outbound policy layer rather than scattered one-off checks
- provider credentials are no longer architecturally tied only to plain env reads
- leaked secrets can be warned/redacted/blocked before persistence
- maintainers can inspect security posture with one CLI command

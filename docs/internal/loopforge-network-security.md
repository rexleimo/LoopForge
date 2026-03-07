# LoopForge Network & Output Security Notes

This note is internal-only and describes the current trust boundary for outbound access, provider secrets, and tool-output handling.

## Trust boundary

LoopForge now applies three additive layers:

1. **Workspace sandbox** for filesystem and shell tools.
2. **Outbound policy checks** for networked tools.
3. **Leak guard** for tool output before it is stored or replayed into later model turns.

These layers are intentionally independent so one missing policy does not disable the others.

## Provider secrets

Current behavior is intentionally simple:

- secrets resolve from host environment variables via `security.secrets.mode = "env_first"`
- provider config stores only env var names, not secret values
- `loopforge doctor` reports the current secret-resolution posture

We do **not** yet ship an encrypted local secret store. That remains a future follow-up.

## Outbound policy

When `security.egress.rules` is empty, LoopForge keeps its baseline SSRF/private-network protection and behaves like previous releases.

When one or more egress rules are configured, the following tool paths must match an allow rule before the request proceeds:

- `web_fetch`
- `a2a_send`
- `a2a_discover`
- browser navigation (`browser_navigate` input URL)

Rule matching is based on:

- tool name
- host
- path prefix
- HTTP method

Private/loopback network checks still apply in addition to allowlist rules.

## Leak guard

Leak guard runs on raw tool output before truncation and before the output is:

- persisted as a tool message
- written into `rexos.audit.tool_calls`
- replayed into the next model turn

Supported modes:

- `off`: current historical behavior
- `warn`: keep content, annotate audit metadata
- `redact`: mask detected ranges before persistence and replay
- `enforce`: block the tool result with a stable error

Current detectors are heuristic and lightweight:

- sensitive values discovered from suspicious host env var names
- common token prefixes such as `sk-`, `ghp_`, `github_pat_`, and `AIza`

This is intentionally conservative and should be treated as a guardrail, not a guarantee.

## Audit expectations

Tool audit records may now include a `leak_guard` object with:

- `mode`
- `detectors`
- `redacted`
- `blocked`

The object never stores the matched secret value itself.

## Operator checklist

Before enabling broader networked workflows, verify:

1. `loopforge doctor` reports the intended leak-guard mode
2. required outbound hosts are captured in `security.egress.rules`
3. provider env vars are present in the host environment
4. public docs remain free of internal-only strategy or competitor references

# Security & Sandboxing

LoopForge is built around running LLM-driven tool calls with guardrails.

## Workspace sandbox

The filesystem tools:

- only allow **relative** paths inside the workspace
- reject parent traversal (`..`)
- reject symlink-based escapes

## Shell tool

The shell tool:

- runs inside the workspace directory
- uses a minimal environment
- enforces a timeout

On Windows, it runs via PowerShell; on Unix, via bash.

## Provider secrets

LoopForge stores **env var names**, not provider secret values.

Current secret resolution is controlled by `security.secrets.mode` and today uses:

- `env_first` — resolve provider credentials from the host environment

This keeps API keys out of `~/.loopforge/config.toml` while still making credential lookup explicit.

## Web fetch and outbound allowlists

`web_fetch` defaults to denying loopback/private IP ranges to reduce SSRF risk.

For local testing you can explicitly allow private targets with `allow_private=true`.

You can also add explicit outbound allowlist rules under `security.egress.rules`.
When rules are present, LoopForge requires a match on tool + host + path prefix + HTTP method before these paths proceed:

- `web_fetch`
- A2A requests
- browser navigation entrypoints

Baseline SSRF/private-network checks still apply in addition to allowlist rules.

## Browser tools

LoopForge can run a headless browser via **CDP** by default (no Python), and can also use a legacy Playwright bridge backend.

- `browser_navigate` / `browser_click` / `browser_type` / `browser_press_key` / `browser_wait_for` / `browser_read_page` / `browser_screenshot` / `browser_close`

Security notes:

- `browser_navigate` is SSRF-checked similar to `web_fetch`.
- `browser_read_page` and `browser_screenshot` also enforce the same private-network protections unless you explicitly allow private targets.
- `browser_screenshot` writes only to **workspace-relative** paths (no absolute paths, no `..`, no symlink escapes).

## Leak guard

Tool output can accidentally contain secrets copied from files, env-backed configs, or third-party responses.
LoopForge supports `security.leaks.mode` to reduce that risk before output is persisted or replayed into later model turns.

Modes:

- `off` — historical behavior
- `warn` — annotate likely leaks, but keep raw output
- `redact` — mask detected ranges before audit persistence and model follow-up
- `enforce` — block the tool result with a stable error

The first rollout uses lightweight detectors for suspicious env-backed secrets and common token prefixes.
It is a practical guardrail, not a substitute for least-privilege credentials.

## Operator visibility

Run:

```bash
loopforge doctor
```

Doctor now reports:

- provider secret-resolution mode
- leak-guard mode
- outbound allowlist coverage
- existing config/browser/tooling checks

## Future: approvals

LoopForge has the structure to add “approval hooks” for higher-risk actions (network writes, destructive commands, etc.). This is intentionally conservative by default.

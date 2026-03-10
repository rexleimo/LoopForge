# Config Reference (`~/.loopforge/config.toml`)

LoopForge stores configuration in `~/.loopforge/config.toml` (path kept for compatibility).

## Mental model

Think of the file in four layers:

1. `providers.*` — how to talk to each model provider
2. `router.*` — which provider/model to use for planning, coding, and summary work
3. `security.*` — what network, secret, and leak-guard rules apply around tool execution
4. `skills.*` — how local skills are allowlisted and approved

In practice:

- `loopforge init` creates the baseline config
- `loopforge config validate` checks whether the file parses and the required structure is present
- `loopforge doctor` helps explain why a config is valid-but-still-not-usable in your environment

## MCP servers (per session)

MCP servers are not persisted in `config.toml` yet. Enable them per run:

```bash
loopforge agent run \
  --workspace my-ws \
  --mcp-config mcp-servers.json \
  --prompt "…"
```

## Validate and inspect

```bash
loopforge config validate
loopforge config validate --json
loopforge doctor
```

Use `config validate` for syntax and schema issues.
Use `doctor` for runtime readiness issues such as missing env vars, browser prerequisites, or security posture warnings.

## Minimal starter example

```toml
[providers.ollama]
kind = "openai_compatible"
base_url = "http://127.0.0.1:11434/v1"
api_key_env = ""
default_model = "qwen3:4b"

[router.planning]
provider = "ollama"
model = "default"

[router.coding]
provider = "ollama"
model = "default"

[router.summary]
provider = "ollama"
model = "default"

[security.secrets]
mode = "env_first"

[security.leaks]
mode = "warn"

[skills]
auto_approve_readonly = true
```

This is enough for a local-first Ollama setup. You can harden it later by adding `security.egress.rules` and tightening skills policy.

## Providers

Each provider entry defines:

- `kind`: driver kind (`openai_compatible`, `zhipu_native`, `minimax_native`, `bedrock`, etc.)
- `base_url`: API base URL
- `api_key_env`: environment variable name that contains the API key (empty for local providers)
- `default_model`: default model for `model = "default"`

Example:

```toml
[providers.ollama]
kind = "openai_compatible"
base_url = "http://127.0.0.1:11434/v1"
api_key_env = ""
default_model = "qwen3:4b"
```

### AWS Bedrock (Converse API)

For AWS Bedrock, use `kind = "bedrock"` plus an `aws_bedrock` table:

```toml
[providers.bedrock]
kind = "bedrock"
base_url = ""     # unused for Bedrock
api_key_env = ""  # unused for Bedrock
default_model = "anthropic.claude-3-5-sonnet-20241022-v2:0"

[providers.bedrock.aws_bedrock]
region = "us-east-1"
cross_region = "" # optional
profile = ""      # optional
```

Notes:

- Bedrock uses the AWS SDK credential chain (env vars, shared config, profiles, instance role, etc.).
- `cross_region` (optional) prefixes model ids with `<cross_region>.` when they are not already prefixed.

## Router

Each task kind selects a `(provider, model)` pair. This is how the runtime decides whether planning, coding, and summary turns should use the same model or different ones:

```toml
[router.planning]
provider = "ollama"
model = "default"

[router.coding]
provider = "ollama"
model = "default"

[router.summary]
provider = "ollama"
model = "default"
```

## Security

```toml
[security.secrets]
mode = "env_first"

[security.leaks]
mode = "redact"

[[security.egress.rules]]
tool = "web_fetch"
host = "docs.rs"
path_prefix = "/"
methods = ["GET"]
```

Fields:

- `security.secrets.mode`
  - `env_first`: resolve provider credentials from host environment variables
- `security.leaks.mode`
  - `off`: do nothing extra
  - `warn`: annotate likely secret leaks but keep raw output
  - `redact`: mask detected ranges before persistence and follow-up model calls
  - `enforce`: block the tool result when likely secrets are detected
- `security.egress.rules`
  - when empty, LoopForge keeps baseline SSRF/private-network guards only
  - when non-empty, outbound requests must match an allow rule in addition to baseline guards

Each egress rule contains:

- `tool`: tool name, for example `web_fetch`
- `host`: exact destination host
- `path_prefix`: required URL path prefix
- `methods`: allowed HTTP methods

Current outbound allowlist enforcement applies to `web_fetch`, A2A requests, and browser navigation entrypoints.

## Built-in presets

LoopForge includes common provider presets (names may evolve):

- OpenAI-compatible: `deepseek`, `kimi`, `qwen`, `glm`, `minimax`
- Provider-native: `glm_native`, `minimax_native`, `qwen_native`

## Skills

This section controls local skill policy. It is separate from the normal tool sandbox: a skill can still be blocked even when the underlying workspace is otherwise valid.

```toml
[skills]
allowlist = ["hello-skill", "qa-helper"]
require_approval = false
auto_approve_readonly = true
experimental = false
```

Fields:

- `allowlist`: optional global skill allowlist
- `require_approval`: force approval for non-readonly skills
- `auto_approve_readonly`: when true, readonly skills skip manual approval
- `experimental`: optional flag for rollout messaging

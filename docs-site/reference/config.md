# Config Reference (`~/.loopforge/config.toml`)

LoopForge stores configuration in `~/.loopforge/config.toml` (path kept for compatibility).

## Providers

Each provider entry defines:

- `kind`: driver kind (`openai_compatible`, `zhipu_native`, `minimax_native`, etc.)
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

## Router

Each task kind selects a `(provider, model)` pair:

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

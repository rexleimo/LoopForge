# 配置参考（`~/.loopforge/config.toml`）

LoopForge 把配置存放在 `~/.loopforge/config.toml`（路径为兼容保留）。

## Providers

每个 provider 条目包含：

- `kind`：驱动类型（`openai_compatible`、`zhipu_native`、`minimax_native` 等）
- `base_url`：API base URL
- `api_key_env`：读取 API key 的环境变量名（本地 provider 可为空）
- `default_model`：当路由里写 `model = "default"` 时使用的默认模型名

示例：

```toml
[providers.ollama]
kind = "openai_compatible"
base_url = "http://127.0.0.1:11434/v1"
api_key_env = ""
default_model = "llama3.2"
```

## Router

每个任务类型会选择一个 `(provider, model)`：

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

字段说明：

- `security.secrets.mode`
  - `env_first`：从宿主机环境变量解析 provider 凭证
- `security.leaks.mode`
  - `off`：不增加额外处理
  - `warn`：标记疑似 secret 泄漏，但保留原始输出
  - `redact`：在落盘和后续模型调用前做脱敏
  - `enforce`：检测到疑似 secret 时直接阻断工具结果
- `security.egress.rules`
  - 为空时，LoopForge 只使用基础 SSRF / 私网访问防护
  - 非空时，出站请求除了基础防护外，还必须命中允许规则

每条 egress rule 包含：

- `tool`：工具名，例如 `web_fetch`
- `host`：目标 host 精确匹配
- `path_prefix`：必须命中的 URL path 前缀
- `methods`：允许的 HTTP 方法

当前的出站白名单会应用到 `web_fetch`、A2A 请求和浏览器导航入口。

## 内置 presets

LoopForge 默认包含一些常用 provider presets（名称可能会演进）：

- OpenAI-compatible：`deepseek`、`kimi`、`qwen`、`glm`、`minimax`
- Provider-native：`glm_native`、`minimax_native`、`qwen_native`

## Skills

```toml
[skills]
allowlist = ["hello-skill", "qa-helper"]
require_approval = false
auto_approve_readonly = true
experimental = false
```

字段说明：

- `allowlist`：可选，全局 skill 白名单
- `require_approval`：是否强制对非只读 skill 进行审批
- `auto_approve_readonly`：为 true 时，只读 skill 自动通过
- `experimental`：用于灰度 / 发布提示的实验开关

# 配置参考（`~/.loopforge/config.toml`）

LoopForge 把配置存放在 `~/.loopforge/config.toml`（路径为兼容保留）。

## 心智模型

可以把这个文件理解成四层：

1. `providers.*` —— 每个模型 provider 怎么连
2. `router.*` —— planning / coding / summary 分别走哪个 provider/model
3. `security.*` —— 工具执行时使用什么 secret、网络、leak guard 规则
4. `skills.*` —— 本地 skills 的白名单与审批策略

实际使用时：

- `loopforge init` 会生成基础配置
- `loopforge config validate` 用来检查配置能不能被正确解析、结构是否完整
- `loopforge doctor` 用来解释“配置虽然合法，但当前环境还不能正常工作”的问题

## 验证与检查

```bash
loopforge config validate
loopforge config validate --json
loopforge doctor
```

`config validate` 更适合查语法/结构问题。
`doctor` 更适合查运行时就绪性问题，比如缺少环境变量、浏览器依赖、或安全姿态告警。

## 最小起步示例

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

这已经足够支撑本地优先的 Ollama 起步；后续如果要收紧安全边界，可以继续加 `security.egress.rules` 和更严格的 skills 策略。

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

每个任务类型会选择一个 `(provider, model)`。这就是 runtime 决定 planning、coding、summary 是否走同一模型还是不同模型的方式：

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

这一节控制的是本地 skills 策略。它和普通工具沙盒不是一回事：即使 workspace 本身合法，skill 仍然可能因为审批/白名单策略被拦下。

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

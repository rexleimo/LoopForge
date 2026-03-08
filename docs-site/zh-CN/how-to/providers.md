# Providers 与路由

LoopForge 从 `~/.loopforge/config.toml` 读取 providers 配置，并把每个任务类型（`planning`、`coding`、`summary`）路由到一个 `(provider, model)` 对。

字段级说明见 [配置参考](../reference/config.md)。
想了解这些路由选择在运行期是如何被消费的，可继续看 [Runtime Architecture](../explanation/runtime-architecture.md)。

## 心智模型

可以把 provider 配置理解成三层：

1. `providers.*` 负责定义怎么连接某个 provider：API 类型、base URL、凭证来源、默认模型。
2. `router.*` 负责决定每个任务类型具体走哪个 `(provider, model)`。
3. runtime 会在每次模型调用前读取路由，所以你可以把 planning 保持在本地，同时把 coding 切到更强的云端模型。

实践里，最稳妥的改动后检查是：

```bash
loopforge config validate
loopforge doctor
```

每次改路由后都建议跑一遍。

## 推荐的演进路径

比较稳的默认路径是：

1. **先本地优先** —— 先把所有路由都指向 `ollama`，把流程跑通。
2. **再混合路由** —— 保持 `planning` 和 `summary` 在本地，把 `coding` 切到更强的云端 provider。
3. **最后再更多上云** —— 当你已经接受成本、延迟和安全姿态后，再把更多任务类型切到托管 provider。

这样做的好处是：前期迭代成本低，后期又能平滑提升编码质量。

## 开箱即用的 presets

执行 `loopforge init` 后，`~/.loopforge/config.toml` 默认已经包含常用 providers（可直接改路由使用）：

- 本地：`ollama`
- OpenAI-compatible：`deepseek`、`kimi` / `kimi_cn`、`qwen` / `qwen_cn` / `qwen_sg`、`glm`、`minimax`、`nvidia`
- Provider-native：`qwen_native*`、`glm_native`、`minimax_native`
- 网关：`minimax_anthropic`
- 官方 API：`anthropic`、`gemini`

通常你只需要做两件事：

1. 配好对应的 API key 环境变量（如果需要）
2. 把一个或多个 `[router.*]` 指向你想用的 provider

## 如何选择 provider kind

- `openai_compatible` —— 适合 OpenAI 风格 Chat Completions 接口；Ollama 和很多托管网关都属于这一类。
- `dashscope_native` —— 适合直接使用阿里云 DashScope 原生行为。
- `zhipu_native` —— 适合直接使用智谱 GLM 原生接口与认证方式。
- `minimax_native` —— 适合直接使用 MiniMax 原生接口。
- `anthropic` —— 适合直接使用 Claude，或驱动已支持的兼容网关。
- `gemini` —— 适合直接使用 Google Gemini。

如果你不确定，优先使用 `loopforge init` 生成的 preset，而不是从零手写一个自定义 provider。

## 安全切换 checklist

切换 provider 或模型时，建议按增量方式做：

1. 设置或更新 provider 的 API key 环境变量
2. 如果 endpoint 或默认模型变了，再修改 `providers.<name>`
3. 每次只改一个 `router.*`
4. 跑校验与诊断
5. 先跑一个很小的 agent 任务，再决定是否全面切换

一个实用的 smoke 路径如下：

```bash
loopforge config validate
loopforge doctor
loopforge agent run --workspace loopforge-smoke --prompt "Create hello.txt"
```

如果你是第一次接入新的云端 provider，也建议结合下文的 provider 专项 smoke tests 一起验证。

## Provider kinds

- `openai_compatible`：OpenAI 兼容 Chat Completions（Ollama / DeepSeek / Kimi / …）
- `zhipu_native`：智谱 GLM 原生（内置 JWT 处理）
- `minimax_native`：MiniMax 原生 `text/chatcompletion_v2`
- `dashscope_native`：阿里云 DashScope 原生
- `anthropic`：Claude API
- `gemini`：Google Gemini API

## 示例：本地 Ollama

```toml
[providers.ollama]
kind = "openai_compatible"
base_url = "http://127.0.0.1:11434/v1"
api_key_env = ""
default_model = "llama3.2"

[router.coding]
provider = "ollama"
model = "default"
```

## 示例：GLM（智谱原生）

```toml
[providers.glm_native]
kind = "zhipu_native"
base_url = "https://open.bigmodel.cn/api/paas/v4"
api_key_env = "ZHIPUAI_API_KEY" # 通常是 "id.secret"
default_model = "glm-4"

[router.coding]
provider = "glm_native"
model = "default"
```

!!! tip "智谱 key 格式"
    如果 `ZHIPUAI_API_KEY` 形如 `id.secret`，LoopForge 会自动签发短期 JWT（无需你手动生成 token）。

## 示例：MiniMax（原生）

```toml
[providers.minimax_native]
kind = "minimax_native"
base_url = "https://api.minimax.chat/v1"
api_key_env = "MINIMAX_API_KEY"
default_model = "MiniMax-M2.5"

[router.coding]
provider = "minimax_native"
model = "default"
```

## 示例：NVIDIA NIM（OpenAI 兼容）

```toml
[providers.nvidia]
kind = "openai_compatible"
base_url = "https://integrate.api.nvidia.com/v1"
api_key_env = "NVIDIA_API_KEY"
default_model = "meta/llama-3.2-3b-instruct"

[router.coding]
provider = "nvidia"
model = "default"
```

## 常见路由模式

### 全本地起步

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

### 本地 planning，云端 coding

```toml
[router.planning]
provider = "ollama"
model = "default"

[router.coding]
provider = "glm_native" # 或 minimax_native / deepseek / kimi / qwen_native ...
model = "default"

[router.summary]
provider = "ollama"
model = "default"
```

当你希望统一跟随 provider 默认模型时，使用 `model = "default"`。

## API keys（环境变量）

LoopForge 会从 `api_key_env` 指定的环境变量读取 key。

=== "Bash (macOS/Linux)"
    ```bash
    export DEEPSEEK_API_KEY="..."
    export MOONSHOT_API_KEY="..."
    export DASHSCOPE_API_KEY="..."
    export ZHIPUAI_API_KEY="id.secret"
    export MINIMAX_API_KEY="..."
    export NVIDIA_API_KEY="..."
    ```

=== "PowerShell (Windows)"
    ```powershell
    $env:DEEPSEEK_API_KEY = "..."
    $env:MOONSHOT_API_KEY = "..."
    $env:DASHSCOPE_API_KEY = "..."
    $env:ZHIPUAI_API_KEY = "id.secret"
    $env:MINIMAX_API_KEY = "..."
    $env:NVIDIA_API_KEY = "..."
    ```

## 可选 smoke tests（真实 provider）

这些测试会真实请求 provider endpoint，并且默认标记为 `#[ignore]`：

```bash
# Ollama（OpenAI-compatible）
LOOPFORGE_OLLAMA_MODEL=<your-model> cargo test -p rexos --test ollama_smoke -- --ignored

# GLM（智谱原生）
ZHIPUAI_API_KEY=<id.secret> LOOPFORGE_GLM_MODEL=<model> cargo test -p rexos --test zhipu_smoke -- --ignored

# MiniMax（原生）
MINIMAX_API_KEY=<key> LOOPFORGE_MINIMAX_MODEL=<model> cargo test -p rexos --test minimax_smoke -- --ignored

# NVIDIA NIM（OpenAI-compatible）
NVIDIA_API_KEY=<key> LOOPFORGE_NVIDIA_MODEL=<model> cargo test -p rexos --test nvidia_nim_smoke -- --ignored
```

## Provider 质量报告（适合 nightly）

生成 provider 质量报告（JSON + Markdown）：

```bash
# 只生成计划（不执行测试）
python3 scripts/provider_health_report.py --out-dir .tmp/provider-health

# 执行可用 provider 的 smoke 测试
python3 scripts/provider_health_report.py --out-dir .tmp/provider-health --run
```

产物：
- `.tmp/provider-health/provider-health.json`
- `.tmp/provider-health/provider-health.md`

提示：
- 设置 `ZHIPUAI_API_KEY` / `MINIMAX_API_KEY` / `NVIDIA_API_KEY` 后会自动纳入对应检查。
- 在没有本地 Ollama 的 CI 环境中，可设置：

```bash
export LOOPFORGE_SKIP_OLLAMA_SMOKE=1
```

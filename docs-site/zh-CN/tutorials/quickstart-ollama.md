# 快速开始（Ollama）

## 什么算成功

跑完本教程后，应该满足三件事：Ollama 可访问、`~/.loopforge/config.toml` 合法、并且 `loopforge agent run` 已经在 workspace 里生成了一个产物。

本教程用 Ollama 的 OpenAI 兼容接口在本地跑通 LoopForge。

## 前置条件

- 已安装并启动 Ollama
- Ollama 里至少有一个 **对话模型**（例如 `qwen3:4b`、`llama3.2` 等；只有 embedding 模型是不行的）

查看本机有哪些模型：

```bash
ollama list
```

默认情况下，LoopForge 会在 `~/.loopforge/config.toml` 里使用：

- `providers.ollama.default_model = "llama3.2"`

如果你没有拉取 `llama3.2`，有两种方式：

1) 直接拉取默认模型：

```bash
ollama pull llama3.2
```

2) 或把 LoopForge 切到你已经有的模型（例：`qwen3:4b`）：

```toml
[providers.ollama]
default_model = "qwen3:4b"
```

## 1) 启动 Ollama

```bash
ollama serve
```

## 2) 初始化 LoopForge

会创建：
- `~/.loopforge/config.toml`
- `~/.loopforge/loopforge.db`

```bash
loopforge init
```

## 3) 运行第一次 session

工具调用会被沙盒限制在 `--workspace` 目录内：

=== "macOS/Linux"
    ```bash
    mkdir -p loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt with the word hi"
    cat loopforge-work/hello.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    mkdir loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt with the word hi"
    Get-Content .\loopforge-work\hello.txt
    ```

LoopForge 会输出最终回答，并把稳定的 `session_id` 持久化到 `loopforge-work/.loopforge/session_id`。

## 4) 在同一个 workspace 里续跑（可选）

```bash
loopforge agent run --workspace loopforge-work --prompt "Now append a newline + bye to hello.txt"
```

## 下一步

- Harness 长任务：见 “Harness 长任务”
- Providers 与路由：见 “Providers 与路由”

# CLI 参考

LoopForge 的二进制命令是 `loopforge`。

## 顶层命令

- `loopforge init` — 初始化 `~/.loopforge`（配置 + 数据库）
- `loopforge onboard` — 一键 onboarding（`init` + 配置校验 + `doctor` + 可选首任务）
- `loopforge doctor` — 诊断常见配置问题（配置文件、providers、浏览器、基础依赖）
- `loopforge agent run` — 在 workspace 中运行一次 agent session
- `loopforge channel drain` — 执行一次 outbox drain（投递队列中的消息）
- `loopforge channel worker` — 运行轮询 outbox 的 dispatcher
- `loopforge harness init` — 初始化 harness workspace（持久化产物 + git）
- `loopforge harness run` — 运行一次增量 harness session
- `loopforge skills list|show|doctor|run` — 发现、查看、诊断并执行本地 skills
- `loopforge daemon start` — 启动 HTTP daemon

## 示例

=== "macOS/Linux"
    ```bash
    loopforge init
    loopforge onboard --workspace loopforge-onboard-demo

    mkdir -p loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt"

    mkdir -p loopforge-task
    loopforge harness init loopforge-task --prompt "Initialize a features checklist for refactoring this repo"
    loopforge harness run loopforge-task --prompt "Continue"

    loopforge channel drain

    loopforge daemon start --addr 127.0.0.1:8787
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge init
    loopforge onboard --workspace loopforge-onboard-demo

    mkdir loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt"

    mkdir loopforge-task
    loopforge harness init loopforge-task --prompt "Initialize a features checklist for refactoring this repo"
    loopforge harness run loopforge-task --prompt "Continue"

    loopforge channel drain

    loopforge daemon start --addr 127.0.0.1:8787
    ```

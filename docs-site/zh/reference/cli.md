# CLI 参考

RexOS 提供单个二进制：`rexos`。

## 顶层命令

- `rexos init` — 初始化 `~/.rexos`（配置 + 数据库）
- `rexos agent run` — 在 workspace 中运行一次 agent session
- `rexos harness init` — 初始化 harness workspace（持久化产物 + git）
- `rexos harness run` — 运行一次增量 harness session
- `rexos daemon start` — 启动 HTTP daemon

## 示例

```bash
rexos init

rexos agent run --workspace /tmp/rexos-work --prompt "Create hello.txt"

rexos harness init /tmp/task --prompt "Initialize a features checklist for refactoring this repo"
rexos harness run /tmp/task --prompt "Continue"

rexos daemon start --addr 127.0.0.1:8787
```


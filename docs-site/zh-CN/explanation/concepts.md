# 概念

LoopForge 是给「不是跑一次就完事」的场景用的。

## Workspace

Workspace 就是你的工作目录：

- 工具（文件读写、shell）只能在这个目录里操作
- harness 的产物也放在这里

## 记忆 (SQLite)

LoopForge 会记住：

- 之前的 session
- 聊天记录
- 小的配置状态

存在 `~/.rexos/rexos.db`，下次跑的时候能接上。

## 工具（沙盒里）

Agent 能用的工具：

- `fs_read` / `fs_write` — 读写文件（只能在 workspace 内，不能 `..` 往上翻）
- `shell` — 执行命令（也只能在 workspace 内）
- `web_fetch` — 抓网页（默认防 SSRF）
- `browser_*` — 无头浏览器（通过 CDP）

!!! note "浏览器依赖"
    默认用本地 Chrome/Chromium/Edge（通过 CDP）。

    找不到浏览器的话，设 `REXOS_BROWSER_CHROME_PATH`。

    旧方案：设 `REXOS_BROWSER_BACKEND=playwright` 并安装 Python + Playwright：

    ```bash
    python3 -m pip install playwright
    python3 -m playwright install chromium
    ```

## 模型路由

LoopForge 会把任务分类：

- planning — 规划
- coding — 写代码
- summary — 总结

每种可以走不同的 provider/model。

## Harness（持久化长任务）

Harness 是在上面的工作流：

1. 用 artifacts 初始化 workspace
2. 增量跑 session
3. 用 `init.sh` / `init.ps1` 验证
4. 用 git 提交 checkpoint

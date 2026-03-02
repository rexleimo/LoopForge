# 有界面 smoke check（example.com）

**目标：** 验证 `browser_*` 端到端可用，并在 workspace 里留下证据文件。

另见：[浏览器自动化（Playwright）](../browser-automation.md)。

## 运行

=== "Bash (macOS/Linux)"
    ```bash
    mkdir -p rexos-demo && cd rexos-demo
    export REXOS_BROWSER_HEADLESS=0

    rexos agent run --workspace . --prompt "使用 browser 工具打开 https://example.com，读取页面内容，把 3 条要点写到 notes/example.md，并把截图保存到 .rexos/browser/example.png，然后关闭浏览器。"
    ```

=== "PowerShell (Windows)"
    ```powershell
    mkdir rexos-demo -Force | Out-Null
    cd rexos-demo
    $env:REXOS_BROWSER_HEADLESS = "0"

    rexos agent run --workspace . --prompt "使用 browser 工具打开 https://example.com，读取页面内容，把 3 条要点写到 notes/example.md，并把截图保存到 .rexos/browser/example.png，然后关闭浏览器。"
    ```

## 预期结果

- `notes/example.md`
- `.rexos/browser/example.png`

## 故障排查

- 看不到浏览器窗口：默认是 headless；设置 `REXOS_BROWSER_HEADLESS=0`（或在 `browser_navigate` 传 `headless=false`）。

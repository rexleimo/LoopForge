# 安全与沙盒

LoopForge 的设计核心是：在允许 LLM 驱动工具调用的同时，尽量加上可解释的护栏。

## Workspace 沙盒

文件系统工具：

- 只允许 **相对路径**（相对于 workspace root）
- 拒绝 `..` 目录穿越
- 拒绝通过 symlink 逃逸 workspace

## Shell 工具

shell 工具：

- 在 workspace 内运行
- 环境尽量最小化
- 强制超时

Windows 下使用 PowerShell；Unix 下使用 bash。

## Provider secrets

LoopForge 在配置里只保存 **环境变量名**，不保存 provider 的真实密钥值。

当前通过 `security.secrets.mode` 控制 secret 解析方式，现阶段支持：

- `env_first`：从宿主机环境变量解析 provider 凭证

这样可以把 API key 保持在 `~/.loopforge/config.toml` 之外，同时让凭证来源保持明确。

## Web fetch 与出站白名单

`web_fetch` 默认拒绝访问 loopback/private IP 段，降低 SSRF 风险。

本地测试场景下，你可以显式开启 `allow_private=true` 来允许访问私网目标。

你也可以在 `security.egress.rules` 里配置出站白名单。配置后，以下路径除了基础 SSRF / 私网防护之外，还必须命中允许规则才会放行：

- `web_fetch`
- A2A 请求
- 浏览器导航入口

规则会同时校验 tool、host、path prefix 和 HTTP method。

## 浏览器工具（browser_*）

LoopForge 默认通过 **CDP** 启动并控制无头浏览器（无需 Python），也支持 legacy 的 Playwright bridge 后端。

- `browser_navigate` / `browser_click` / `browser_type` / `browser_press_key` / `browser_wait_for` / `browser_read_page` / `browser_screenshot` / `browser_close`

安全说明：

- `browser_navigate` 默认与 `web_fetch` 类似的 SSRF 防护。
- `browser_read_page` 与 `browser_screenshot` 也会做同样的私网访问保护，除非你显式允许私网目标。
- `browser_screenshot` 只会写入 **workspace 相对路径**（不允许绝对路径、不允许 `..`、不允许通过 symlink 逃逸）。

## Leak guard

工具输出有时会把文件里的敏感内容、env 对应的 secret，或者第三方返回的 token 原样带出来。
LoopForge 支持通过 `security.leaks.mode` 在输出落盘或进入下一轮模型上下文前做一道保护。

模式：

- `off`：保持历史行为
- `warn`：标记疑似泄漏，但仍保留原始输出
- `redact`：在写入审计和后续模型上下文前做脱敏
- `enforce`：一旦检测到疑似 secret，直接阻断该工具结果

第一版使用轻量级探测器：宿主环境中的可疑 secret 值，以及常见 token 前缀。
它是实用护栏，不是替代最小权限和凭证治理的万能方案。

## 运维可见性

运行：

```bash
loopforge doctor
```

现在 doctor 会额外报告：

- provider secret 解析模式
- leak guard 模式
- 出站白名单覆盖情况
- 其他已有的配置 / 浏览器 / 工具链检查

## 未来：审批（Approvals）

LoopForge 的结构允许未来加入“审批钩子”，对更高风险的行为（网络写入、破坏性命令等）进行确认。

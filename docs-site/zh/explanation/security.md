# 安全与沙盒

RexOS 的设计核心是：在允许 LLM 驱动工具调用的同时，尽量加上可解释的护栏。

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

## Web fetch（SSRF 防护）

`web_fetch` 默认拒绝访问 loopback/private IP 段，降低 SSRF 风险。

本地测试场景下，你可以显式开启 `allow_private=true` 来允许访问私网目标。

## 未来：审批（Approvals）

RexOS 的结构允许未来加入“审批钩子”，对更高风险的行为（网络写入、破坏性命令等）进行确认。


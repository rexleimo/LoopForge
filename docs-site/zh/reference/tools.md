# 工具参考

RexOS 对 agent runtime 暴露了一组小而清晰的工具集。

## `fs_read`

读取 **相对于 workspace root** 的 UTF-8 文本文件。

- 拒绝绝对路径
- 拒绝 `..` 目录穿越
- 拒绝 symlink 逃逸

## `fs_write`

写入 **相对于 workspace root** 的 UTF-8 文本文件（必要时创建父目录）。

沙盒规则与 `fs_read` 相同。

## `shell`

在 workspace 内执行 shell 命令：

- Unix：通过 `bash -c`
- Windows：通过 PowerShell

RexOS 会强制超时，并使用尽量最小的环境。

## `web_fetch`

抓取一个 HTTP(S) URL，并返回一小段响应体。

默认拒绝 loopback/private IP 段（基础 SSRF 防护）。本地测试可用 `allow_private=true` 显式放开。


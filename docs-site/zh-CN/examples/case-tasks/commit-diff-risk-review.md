# 最近提交风险评审

**目标：** 对近期改动做按严重级别排序的风险评审。

## 运行

1) 先 `cd` 到目标仓库。

2) 执行：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "审查这个仓库最近 5 个提交和变更文件。写 notes/commit-diff-risk-review.md，按 High/Medium/Low 三个等级列风险。每条都要包含：风险原因、可能用户影响、一个可执行的测试或检查命令。内容要简洁、技术导向。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "审查这个仓库最近 5 个提交和变更文件。写 notes/commit-diff-risk-review.md，按 High/Medium/Low 三个等级列风险。每条都要包含：风险原因、可能用户影响、一个可执行的测试或检查命令。内容要简洁、技术导向。"
    ```

## 预期产物

- `notes/commit-diff-risk-review.md`

!!! tip
    如果你想更偏 code review，可以加一句："重点关注行为回归"。

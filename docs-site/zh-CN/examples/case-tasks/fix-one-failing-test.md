# 修复一个失败测试（最小改动）

**目标：** 找出一个失败测试，用最小安全改动修复，并记录修复过程。

## 运行

1) 先 `cd` 到目标仓库。

2) 执行：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "识别这个仓库的主测试命令（如 Cargo/npm/pnpm/pytest/go test），先跑一遍测试，只选择一个失败用例进行修复，并保持改动最小。然后只重跑相关测试确认修复。最后写 notes/fix-one-failing-test.md，包含：失败现象、根因、修改文件、验证命令、剩余风险。如果没有失败测试，也要写 notes/fix-one-failing-test.md 说明未发现失败并给出你执行的命令。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "识别这个仓库的主测试命令（如 Cargo/npm/pnpm/pytest/go test），先跑一遍测试，只选择一个失败用例进行修复，并保持改动最小。然后只重跑相关测试确认修复。最后写 notes/fix-one-failing-test.md，包含：失败现象、根因、修改文件、验证命令、剩余风险。如果没有失败测试，也要写 notes/fix-one-failing-test.md 说明未发现失败并给出你执行的命令。"
    ```

## 预期产物

- `notes/fix-one-failing-test.md`
- 一处小而聚焦的代码/测试改动（当确实存在失败测试时）

!!! tip "控制范围"
    可以在 prompt 里补一句："不要重构无关代码"。

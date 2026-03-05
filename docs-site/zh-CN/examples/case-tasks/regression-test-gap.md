# 回归测试缺口地图

**目标：** 识别高风险但覆盖不足的路径，并给出优先测试清单。

## 运行

1) 先 `cd` 到目标仓库。

2) 执行：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "检查 src/（或等价代码目录）、现有测试目录和近期改动。写 notes/regression-test-gap.md，包含：1) 可能回归的关键行为 2) 当前测试覆盖信号 3) Top 5 缺失的回归测试（含建议测试名和断言目标）4) 你现在最先会补的一个测试及原因。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "检查 src/（或等价代码目录）、现有测试目录和近期改动。写 notes/regression-test-gap.md，包含：1) 可能回归的关键行为 2) 当前测试覆盖信号 3) Top 5 缺失的回归测试（含建议测试名和断言目标）4) 你现在最先会补的一个测试及原因。"
    ```

## 预期产物

- `notes/regression-test-gap.md`

!!! tip
    可以补一句："优先用户可感知故障"，让结果更实用。

# 配置健康检查

**目标：** 验证配置和 doctor 状态，并生成可执行的修复建议。

## 运行

1) 先执行健康检查命令：

=== "macOS/Linux"
    ```bash
    loopforge config validate | tee notes/config-validate.txt
    loopforge doctor | tee notes/doctor.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge config validate | Tee-Object -FilePath notes/config-validate.txt
    loopforge doctor | Tee-Object -FilePath notes/doctor.txt
    ```

2) 让 Agent 产出总结：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "读取 notes/config-validate.txt 和 notes/doctor.txt。写 notes/config-health-check.md，包含：1) 当前状态 2) 阻塞错误 3) 非阻塞告警 4) 修复前三个问题的具体命令。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "读取 notes/config-validate.txt 和 notes/doctor.txt。写 notes/config-health-check.md，包含：1) 当前状态 2) 阻塞错误 3) 非阻塞告警 4) 修复前三个问题的具体命令。"
    ```

## 预期产物

- `notes/config-validate.txt`
- `notes/doctor.txt`
- `notes/config-health-check.md`

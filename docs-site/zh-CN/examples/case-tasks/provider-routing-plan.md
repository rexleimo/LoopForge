# Provider 路由方案（本地 / 团队 / CI）

**目标：** 为不同运行模式产出一份可落地的 provider 路由方案。

## 运行

1) 先 `cd` 到目标仓库。

2) 执行：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "阅读 providers 与 routing 相关文档（例如 docs-site/how-to/providers.md 和 docs-site/reference/config.md）。写 notes/provider-routing-plan.md，设计 3 套配置：本地开发（低成本）、团队默认（平衡）、CI/发布检查（稳定优先）。每套都给出 provider/model 选择、fallback 策略，以及 cost/latency/quality 的权衡说明。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "阅读 providers 与 routing 相关文档（例如 docs-site/how-to/providers.md 和 docs-site/reference/config.md）。写 notes/provider-routing-plan.md，设计 3 套配置：本地开发（低成本）、团队默认（平衡）、CI/发布检查（稳定优先）。每套都给出 provider/model 选择、fallback 策略，以及 cost/latency/quality 的权衡说明。"
    ```

## 预期产物

- `notes/provider-routing-plan.md`

!!! note
    先产出方案再改配置，避免一次性改动过大。

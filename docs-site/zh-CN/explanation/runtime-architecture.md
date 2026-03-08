# 运行时架构

这页适合已经跑过 quickstart、想进一步理解 LoopForge 内部结构的读者。

## 一句话版本

LoopForge 由一组边界清晰的 Rust crate 组成：

- `loopforge-cli` —— 用户入口 CLI（`loopforge init`、`loopforge onboard`、`loopforge agent run`、`loopforge doctor`）
- `rexos-runtime` —— agent runtime：session、运行时保留工具、workflow、audit、approval、leak guard、状态编排
- `rexos-tools` —— 独立工具执行层：文件、shell、浏览器、网页、PDF、媒体、进程
- `rexos-memory` —— 持久化存储：聊天记录、KV 状态、工具调用痕迹、运行时记录
- `rexos-llm` —— provider driver 与模型路由辅助层
- `rexos-kernel` —— 配置、安全、路由、路径等共享基础能力
- `rexos-daemon` —— 可选的 HTTP daemon 能力
- `rexos-harness` —— 长任务 workspace 初始化与 checkpoint 流程

目标很直接：保持本地优先、可审计、并且让系统在不断演进时仍然可读、可拆、可维护。

## 一次 session 是怎么流动的

一次典型的 `loopforge agent run` 会经过下面的链路：

1. **CLI 层** 读取配置、workspace 参数和路由选择。
2. **Runtime 层** 读取 session 历史并准备下一次模型请求。
3. **LLM driver** 把统一请求转换成 provider 的具体 API 调用。
4. 如果模型请求工具，**tool processing** 会先做白名单检查、approval 检查和 leak guard 检查。
5. 工具会走两种路径之一：
   - 由 `rexos-runtime` 实现的 **runtime-managed tools**（适合 task、hand、workflow、schedule、knowledge、outbox 这类有状态能力），或
   - 由 `rexos-tools` 执行的 **standalone tools**（适合 shell、文件、浏览器、网络、PDF、媒体、进程等能力）。
6. 工具输出、audit、ACP event 会写入 **memory**。
7. 工具结果再回到对话中，直到模型给出最终回答。

## 为什么 `rexos-runtime` 是核心

`rexos-runtime` 是有状态编排核心。
它负责的能力包括：

- session 生命周期
- 高风险工具的 approval 检查
- 在工具输出进入后续模型上下文前做 leak guard
- agents、hands、tasks、schedules、workflows、knowledge、channels 等 runtime-managed tools
- ACP events 与 delivery checkpoints
- tool / skill 的 audit 持久化

最近的重构重点不是改产品行为，而是把这个 crate 按关注点拆成更小的模块，让维护者可以一次只理解一类问题。

## 为什么 `rexos-tools` 要保持独立

`rexos-tools` 更像“可复用的工具执行层”。
它主要关注稳定的工具定义与沙盒执行，例如：

- workspace 内文件读写
- workspace 内 shell 执行
- 基于 CDP 的浏览器自动化
- 带 SSRF / egress policy 检查的 web fetch
- PDF 文本抽取
- 进程与媒体相关工具

把工具层和运行时编排层拆开，能让工具更容易测试和复用，而把策略与状态留在 runtime。

## 安全模型速览

LoopForge 不是只靠一个“大开关”保证安全，而是多层防护：

- **workspace sandbox**：约束文件与 shell 工具
- **egress policy + SSRF 检查**：约束外部 HTTP 与浏览器入口
- **approval gate**：限制更高风险的工具调用
- **leak guard**：阻止敏感输出被持久化或回放给模型
- **audit records + ACP events**：便于事后排查与审计

更偏用户视角的说明，请看 [安全与沙盒](security.md)。

## 最近这些重构到底改了什么

架构方向本身没有改变。
真正变化的是 **代码组织方式**：

- 更少的超长单文件
- 更多按关注点拆分的模块边界
- 更薄的顶层文件，只负责组织子模块
- storage、lifecycle、policy、execution 更明确地分层

这样做的好处是：在保持行为不变的前提下，让后续阅读、review、回归验证和继续拆分都更容易。

## 想继续深入？

建议接着看：

- [概念](concepts.md)
- [安全与沙盒](security.md)
- [Providers 与路由](../how-to/providers.md)
- [工具参考](../reference/tools.md)
- [CLI 参考](../reference/cli.md)
- [Harness 长任务工作流](../tutorials/harness-long-task.md)

# LoopForge 可读性与工程化重构 OKR（2026-03）

这是一份**仓库内部** OKR 记录。
不要把它接入 `docs-site/` 或 `mkdocs.yml`，也不要发布到公开文档站点。

## 状态

- 状态：已完成本轮主目标
- 范围：`meos` 仓库
- 时间窗口：2026-03 上旬连续迭代

## O：目标

把 `meos` 从“单文件堆很多实现细节”的状态，持续整理成一个：

- 更容易阅读
- 职责更单一
- 模块边界更清晰
- 更符合软件工程实践
- 更适合后续持续迭代

的本地优先 agent runtime / CLI 项目。

核心原则：

1. **尽量零行为变更**，优先做机械拆分与封装整理
2. **先拆热点文件**，把大文件变成“薄入口 + 小模块”
3. **公共文档只服务用户和开发者**，维护者地图、竞品分析、策略材料只留内部
4. **每轮都必须验证**，不能只凭感觉说“应该没问题”

## 范围边界

### In Scope

- `rexos-runtime` 可读性重构
- `loopforge-cli` 可读性重构
- 公开文档的结构化增强与心智模型统一
- 内部计划、内部维护文档、内部边界规则补齐

### Out of Scope

- 直接修改 `openfang`
- 直接修改 `.tmp/openclaw`
- 把竞品分析、借鉴说明、内部策略写进公开 docs 站点
- 为了“重构而重构”去引入不必要的新抽象或大行为变化

## KR：关键结果

### KR1：运行时热点文件持续拆薄

目标：把仍然承担多种职责的 runtime 文件继续拆成按 concern 分组的小模块。

本轮代表性结果：

- `crates/rexos-runtime/src/session_runner/tool_dispatch.rs`
- `crates/rexos-runtime/src/workflow/execution.rs`
- `crates/rexos-runtime/src/outbox.rs`
- `crates/rexos-runtime/src/leak_guard/detect.rs`
- `crates/rexos-runtime/src/tool_calls/parse.rs`

已达成的拆分模式：

- `tool_dispatch/*`：按 memory / agents_hands / tasks_scheduling / workflow_knowledge 分拆
- `workflow/execution/*`：按 steps / events / result 分拆
- `outbox/*`：按 dispatcher / queue / events / delivery / store 分拆
- `leak_guard/detect/*`：按 env / search 分拆
- `tool_calls/parse/*`：按 json / scan 分拆

### KR2：CLI 剩余热点与编译噪音收尾

目标：把 CLI 中还偏厚的 doctor / onboard 相关文件继续拆分，并清掉当前可见的编译噪音。

本轮代表性结果：

- `crates/loopforge-cli/src/doctor/actions.rs`
- `crates/loopforge-cli/src/doctor/probes/config/runtime.rs`
- `crates/loopforge-cli/src/onboard.rs`

已达成的拆分模式：

- `doctor/actions/*`：按 summary / next_actions 分拆
- `doctor/probes/config/runtime/*`：按 router / providers / security / ollama 分拆
- `onboard.rs`：改成更干净的模块边界入口，测试专用导出使用 `#[cfg(test)]`

### KR3：公共文档统一成更清晰的用户路径

目标：让公开文档更像“用户能顺着走完”的产品文档，而不是零散说明堆积。

本轮达成的统一方向：

- 先告诉用户**什么时候看这页**
- 再告诉用户**推荐路径**或**成功标准**
- 最后给出**验证命令**和下一步链接

代表性页面：

- `docs-site/reference/cli.md`
- `docs-site/how-to/providers.md`
- `docs-site/how-to/install.md`
- `docs-site/how-to/browser-automation.md`
- `docs-site/tutorials/new-user-walkthrough.md`
- `docs-site/tutorials/skills-quickstart.md`
- 对应 `zh-CN` 页面同步更新

### KR4：公开信息边界明确且可执行

目标：确保“哪些内容能公开、哪些只能内部可见”不再靠口头记忆。

已达成：

- 公开边界规则沉淀为内部文档：`docs/internal/public-docs-boundary.md`
- 运行时维护者地图沉淀为内部文档：`docs/internal/runtime-module-map.md`
- 竞品分析保持在：`docs/internal/competitive/`
- 发布检查会拦截公开输入里的竞品内容

### KR5：验证成为完成定义的一部分

目标：不再用“应该没问题”来判断完成，而是以新鲜验证结果作为完成依据。

本轮使用过的完成验证：

- `cargo fmt --all --check`
- `cargo run --quiet --bin loopforge -- --help`
- `python3 -m mkdocs build --strict`
- `cargo test --workspace --locked`

## 完成定义（Definition of Done）

当以下条件同时满足时，可认为本轮 OKR 主目标完成：

1. 主要热点文件已经继续拆薄，不再集中承载过多职责
2. 公开文档已经形成稳定的用户阅读路径
3. 维护者专用信息与公开信息边界清晰
4. 工作区在新鲜验证下通过格式、构建/帮助输出、文档严格构建和全量测试

按这个定义，本轮主目标已经满足。

## 代表性成果索引

### 代码

- `crates/rexos-runtime/src/session_runner/tool_dispatch.rs`
- `crates/rexos-runtime/src/workflow/execution.rs`
- `crates/rexos-runtime/src/outbox.rs`
- `crates/rexos-runtime/src/leak_guard/detect.rs`
- `crates/rexos-runtime/src/tool_calls/parse.rs`
- `crates/loopforge-cli/src/doctor/actions.rs`
- `crates/loopforge-cli/src/doctor/probes/config/runtime.rs`
- `crates/loopforge-cli/src/onboard.rs`

### 文档

- `docs-site/reference/cli.md`
- `docs-site/how-to/providers.md`
- `docs-site/how-to/install.md`
- `docs-site/how-to/browser-automation.md`
- `docs-site/tutorials/new-user-walkthrough.md`
- `docs-site/tutorials/skills-quickstart.md`
- `docs/internal/public-docs-boundary.md`
- `docs/internal/runtime-module-map.md`

### 计划记录

- `docs/plans/2026-03-08-final-okrs-sweep.md`
- 以及同日其他 `docs/plans/2026-03-08-*.md` 分解记录

## 本轮没有纳入完成标准的事项

这些不是 blocker，但可以作为下一轮候选：

- 对公开文档做一次更细的语气统一审校
- 对测试文件是否进一步按主题拆分做专项优化
- 继续寻找新的 bounded hotspots 做下一轮机械拆分

## 下一轮建议（非本轮 blocker）

下一轮如果继续做工程化质量 OKR，建议从以下方向二选一：

1. **测试可读性专项**
   - 把偏长测试文件按主题继续分组
   - 强化“一个测试文件只讲一种行为”的阅读体验

2. **开发者体验专项**
   - 继续优化 CLI 帮助输出
   - 补更多“命令怎么选”的 reference/how-to 导航
   - 整理内部维护文档索引

## 相关内部文档

- 公开边界规则：`docs/internal/public-docs-boundary.md`
- Runtime 维护者地图：`docs/internal/runtime-module-map.md`
- 竞品分析归档：`docs/internal/competitive/`

# Agent Team 作战手册

LoopForge 在团队里的最佳用法，不是“单线程长对话”，而是按 **Agent Team 分工**协作。

这页给出一个可复用、可演示的团队运行模型。

## 团队分工拓扑

| 角色 | 核心输出 | 建议产物 |
|---|---|---|
| Planner Agent | 范围、里程碑、风险图 | `notes/plan.md` |
| Builder Agent | 通过验证的实现 | 代码 diff + 测试结果 |
| Reviewer Agent | findings-first 评审结论 | `notes/review.md` |
| Release Agent | 发布就绪与门禁结果 | `notes/release-check.md` |

## 运行闭环

1. 需求接入与规划
   - 明确目标、约束、验收标准。
   - 先产出 `notes/plan.md`。
2. 分片执行
   - 小步实现，每片都做验证并 checkpoint。
   - 每片至少沉淀一个产物（报告、检查单、修复记录）。
3. 评审
   - 合并或发布前执行 findings-first 评审。
   - 记录严重级别、影响文件、处理决策。
4. 发布门禁
   - 执行 `loopforge release check --tag vX.Y.Z`。
   - 版本号、changelog、CI 全绿再发布。

## 命令基线

```bash
# 启动与对齐
loopforge onboard --workspace loopforge-team-demo --starter workspace-brief

# Builder 执行
loopforge agent run --workspace loopforge-team-demo --prompt "Implement task from notes/plan.md"

# 发布门禁
loopforge release check --tag v1.3.0
```

## 可复制角色 Prompt

- Planner："给出 5 步实现计划，包含风险和验证命令，写入 `notes/plan.md`。"
- Builder："执行 `notes/plan.md` 的第 1 步，保持行为不变，并输出简短验证报告。"
- Reviewer："对当前 diff 做 findings-first 评审（严重级别 + 文件/行），再列剩余风险。"
- Release："执行发布就绪检查，并把每个门禁的 pass/fail 写入 `notes/release-check.md`。"

## 团队演示建议展示的 3 个指标

- 首个“已验证产物”的产出时长
- 每个分片的验证通过率
- release-check 通过率趋势

这三项提升，通常意味着 Agent Team 产能和稳定性都在同步提升。

## 下一步阅读

- [Harness 长任务工作流](../tutorials/harness-long-task.md)
- [案例任务库](../examples/case-tasks/index.md)
- [发布就绪审计任务](../examples/case-tasks/release-readiness-audit.md)

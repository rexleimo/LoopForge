# 5 分钟可见结果

这页只回答一个问题：
LoopForge 今天到底能帮我产出什么？

## 前置准备

```bash
ollama serve
loopforge init
```

## 结果 1：新 workspace 产出第一份文档

```bash
mkdir -p demo-work
loopforge agent run --workspace demo-work --prompt "Create notes/hello.md with a short intro to this workspace and 3 next actions."
```

预期产物：
- `demo-work/notes/hello.md`

## 结果 2：修一个失败测试并输出报告

```bash
loopforge agent run --workspace . --prompt "Run tests. Fix one failing test. Re-run that test. Write notes/fix-report.md with root cause and patch summary."
```

预期产物：
- `notes/fix-report.md`

## 结果 3：输出发布就绪评估

```bash
loopforge agent run --workspace . --prompt "Read CHANGELOG.md and recent commits. Create notes/release-readiness.md with: risks, blockers, go/no-go recommendation, and next actions."
```

预期产物：
- `notes/release-readiness.md`

## 为什么重要

这些不是一次性聊天回复，而是可持续复查、可提交、可交接的工程产物。

下一步：
- [案例任务库](../examples/case-tasks/index.md)
- [为什么是 LoopForge](../explanation/why-loopforge.md)

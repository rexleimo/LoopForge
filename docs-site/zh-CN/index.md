<div class="rexos-hero" markdown>

# LoopForge

**你的个人研发助理（Personal AI Engineer）。**

从一句提示到可交付产物：失败测试修复报告、发布检查单、调研 Memo、可复现 checkpoint。

[5 分钟开始产出](tutorials/five-minute-outcomes.md){ .md-button .md-button--primary }
[它能做什么](examples/case-tasks/index.md){ .md-button }
[为什么是 LoopForge](explanation/why-loopforge.md){ .md-button }
[简单介绍](blog/what-is-loopforge.md){ .md-button }
[个人研发助理定位](blog/personal-ai-engineer.md){ .md-button }

<p class="rexos-muted">
OpenClaw 更像个人生活助理。LoopForge 的定位是工程交付型 AI 助理：本地优先、可复现、可审计。
</p>

</div>

> LoopForge 是当前产品名。CLI 命令是 `loopforge`，配置目录还是 `~/.loopforge`。

<div class="grid cards" markdown>

- :material-hammer-wrench: **修一个失败测试**
  让 LoopForge 自动跑测试、修一个失败用例，并输出 `notes/fix-report.md`。
  [可复制任务](examples/case-tasks/fix-one-failing-test.md)

- :material-clipboard-check: **发布就绪审计**
  基于提交、测试、变更日志生成 go/no-go 检查清单。
  [可复制任务](examples/case-tasks/release-readiness-audit.md)

- :material-file-document-edit: **路由与成本方案**
  输出 provider/model 路由建议、取舍和回滚建议。
  [可复制任务](examples/case-tasks/provider-routing-plan.md)

- :material-history: **可复现推进**
  固化流程：修改 -> 验证 -> checkpoint。
  [Harness 工作流](tutorials/harness-long-task.md)

</div>

## 3 个高价值快产出

=== "1) 先跑通一次"
    ```bash
    ollama serve
    loopforge init
    mkdir -p my-work
    loopforge agent run --workspace my-work --prompt "Create notes/hello.md with a short project intro."
    ```

=== "2) 修复一个失败测试"
    ```bash
    loopforge agent run --workspace . --prompt "Run tests. Fix one failing test. Re-run that test. Write notes/fix-report.md with root cause and patch summary."
    ```

=== "3) 输出发布审计"
    ```bash
    loopforge agent run --workspace . --prompt "Read CHANGELOG.md and recent commits. Create notes/release-readiness.md with: risks, blockers, go/no-go, and next actions."
    ```

## 我们的定位

- 当你的核心目标是“工程交付效率 + 可复现执行”，选 **LoopForge**。
- 当你的核心目标是“生活助理式交互体验”，选个人助手类产品。
- 当你的核心目标是“多渠道运营覆盖”，选平台型产品。

详细说明见：[为什么是 LoopForge](explanation/why-loopforge.md)。

## 下一步

- [5 分钟产出教程](tutorials/five-minute-outcomes.md)
- [个人研发助理定位文](blog/personal-ai-engineer.md)
- [案例任务库](examples/case-tasks/index.md)
- [Provider 配置](how-to/providers.md)
- [常见问题](how-to/faq.md)

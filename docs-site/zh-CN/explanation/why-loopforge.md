# 为什么是 LoopForge

LoopForge 的目标不是做“泛生活助手”。
它的定位是 **个人研发助理（Personal AI Engineer）**，聚焦软件交付。

## 定位对比

| 产品形态 | 核心价值 |
|---|---|
| 个人生活助理 | 日常聊天与生活类任务 |
| 渠道运营平台 | 多渠道运营与适配器广度 |
| LoopForge | 可复现工程流程与可交付产物 |

## LoopForge 优先优化什么

1. 可复现执行（`修改 -> 验证 -> checkpoint`）
2. 本地优先起步（先 Ollama，按需切云端）
3. 持久化产物（报告、检查单、总结文档进入 workspace）
4. 可审计轨迹（session + memory + 文件产物）

## 对团队意味着什么

- 工程交接更容易
- 回滚更稳，因为 checkpoint 明确
- 评审更直接，因为输出是文件，不只是对话

## 10 分钟评估方式

依次跑这三条：
1. [5 分钟可见结果](../tutorials/five-minute-outcomes.md)
2. [修复一个失败测试](../examples/case-tasks/fix-one-failing-test.md)
3. [发布就绪审计](../examples/case-tasks/release-readiness-audit.md)

# OpenClaw 风口：LoopForge 优化与 X 推广作战页

最后更新：**2026 年 3 月 5 日**。

本文把竞品信号转成可执行动作：LoopForge 该先优化什么，以及这周在 X 上该怎么发。

## 证据来源

- OpenClaw GitHub README（定位、onboard 向导、渠道覆盖）：  
  https://github.com/openclaw/openclaw
- OpenClaw 文档（getting started / wizard / onboarding）：  
  https://docs.openclaw.ai/start/getting-started  
  https://docs.openclaw.ai/start/wizard
- LoopForge 当前文档（CLI 能力与可靠性基线）：  
  ../reference/cli.md  
  ../explanation/reliability-baseline.md

## OpenClaw 这波热度说明了什么

1. 上手漏斗质量直接决定增长效率（`openclaw onboard` 作为默认入口）。
2. 场景文档足够多，评估成本就会下降。
3. 高频更新与文档联动，会持续累积信任。

推断：OpenClaw 的势能是真实存在的，但它主打的是“个人助手覆盖广度”。LoopForge 应该借同样的增长机制，继续强化“工程可复现交付”这条主线。

## LoopForge 优化清单（P0/P1）

### P0（1-2 周内）

1. 固定周更 onboarding 可靠性数据。
   - 产物：由 `scripts/onboard_metrics_report.py` 生成 `onboard-report.md`。
2. 把案例任务从“示例集合”升级为“行业任务包”。
   - 第一批：仓库迁移、测试修复、发布审计。
3. 每个功能发布都配套 X 素材。
   - 规则：每次 release note 附 2 条短帖 + 1 条串帖开头。

### P1（2-4 周内）

1. 增加“失败到恢复”的演示路径（doctor -> fix -> rerun）。
2. 上线一个轻量 proof gallery，展示命令和结果证据。
3. 固化周报指标卡（首跑耗时、首任务成功率、失败类别 TopN）。

## X 发帖长度规则（防超字数）

1. 推荐范围：**220-260**（留改稿余量）。
2. 硬上限：**280**。
3. 发帖前本地检查：

```bash
python3 scripts/x_post_lint.py --file docs/marketing/openclaw-trend-x-posts.zh-CN.txt --limit 280 --warn-at 260
```

这个 lint 会按 URL=23 字符和东亚宽字符规则做发布前预检。

## 可直接发布的文案包

- 中文：`docs/marketing/openclaw-trend-x-posts.zh-CN.txt`
- 英文：`docs/marketing/openclaw-trend-x-posts.en.txt`

两个文件都用 `---` 分隔，每一段就是一条可发布草稿。

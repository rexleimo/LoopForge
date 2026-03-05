# OpenClaw Trend: LoopForge Optimization + X Playbook

Last updated: **March 5, 2026**.

This page turns competitor signals into concrete execution for LoopForge: what to improve next, and what to post on X this week.

## Source anchors

- OpenClaw GitHub README (positioning, onboarding wizard, channel breadth):  
  https://github.com/openclaw/openclaw
- OpenClaw docs (getting started / wizard / onboarding):  
  https://docs.openclaw.ai/start/getting-started  
  https://docs.openclaw.ai/start/wizard
- LoopForge CLI and onboarding baseline docs:  
  ../reference/cli.md  
  ../explanation/reliability-baseline.md

## What OpenClaw validates in the market

1. Onboarding funnel quality is a growth engine (`openclaw onboard` as default path).
2. Scenario coverage and links reduce evaluation friction.
3. Public docs + release rhythm create compounding trust.

Inference: OpenClaw momentum is real, but its center of gravity is personal assistant breadth. LoopForge should use the same growth mechanics while keeping our engineering-delivery positioning.

## LoopForge optimization backlog (P0/P1)

### P0 (ship in 1-2 weeks)

1. Publish onboarding reliability counters weekly.
   - Artifact: `onboard-report.md` from `scripts/onboard_metrics_report.py`.
2. Expand copy/paste task packs from "examples" into vertical bundles.
   - Start with: repo migration, test stabilization, release audit.
3. Add X-ready snippets for each feature release.
   - Every release note includes 2 short posts and 1 thread opener.

### P1 (ship in 2-4 weeks)

1. Add "from failure to recovery" walkthroughs (doctor -> fix -> rerun).
2. Build a small "proof gallery" page with command/output evidence.
3. Introduce weekly benchmark cards (time-to-first-success, first-task success rate).

## X posting rule for character limits

1. Target length: **220-260** (buffer for edits/mentions).
2. Hard limit: **280**.
3. Validate drafts before publishing:

```bash
python3 scripts/x_post_lint.py --file docs/marketing/openclaw-trend-x-posts.en.txt --limit 280 --warn-at 260
```

The linter uses URL weight 23 and East Asian width rules for a practical preflight check.

## Ready-to-post copy pack

- English pack: `docs/marketing/openclaw-trend-x-posts.en.txt`
- Chinese pack: `docs/marketing/openclaw-trend-x-posts.zh-CN.txt`

Use the file blocks split by `---` as one-post units.

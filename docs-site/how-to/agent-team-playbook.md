# Agent Team Playbook

LoopForge is most effective when you run it as an **agent team**, not a single long chat thread.

This page gives you a practical team operating model you can demo and reuse.

## Team Topology

| Role | Primary output | Suggested artifact |
|---|---|---|
| Planner Agent | Scope, milestones, risk map | `notes/plan.md` |
| Builder Agent | Verified implementation | code diff + test output |
| Reviewer Agent | Findings-first review | `notes/review.md` |
| Release Agent | Release readiness + publish gate | `notes/release-check.md` |

## Operating Loop

1. Intake and planning
   - Capture objective, constraints, and acceptance criteria.
   - Run one planning task and write `notes/plan.md`.
2. Execution
   - Build in small slices and checkpoint each verified slice.
   - Keep one artifact per slice (report, checklist, or fix memo).
3. Review
   - Run findings-first review before merge/release.
   - Record severity, impacted files, and decision.
4. Release gate
   - Run `loopforge release check --tag vX.Y.Z`.
   - Ensure version/changelog/CI gates are green before publish.

## Command Baseline

```bash
# bootstrap
loopforge onboard --workspace loopforge-team-demo --starter workspace-brief

# builder execution
loopforge agent run --workspace loopforge-team-demo --prompt "Implement task from notes/plan.md"

# release gate
loopforge release check --tag v1.3.0
```

## Copy/Paste Role Prompts

- Planner: "Create a 5-step implementation plan with risks and verification commands. Write `notes/plan.md`."
- Builder: "Execute step 1 from `notes/plan.md`, keep behavior unchanged, and output a short verification report."
- Reviewer: "Review current diff with findings first (severity + file/line), then list residual risks."
- Release: "Run release readiness checks and write `notes/release-check.md` with pass/fail per gate."

## What To Show In Team Demos

- Time to first verified artifact
- Verification pass rate per slice
- Release-check pass/fail trend

If those three metrics improve, your agent team is getting stronger in the right way.

## Next Links

- [Harness long task workflow](../tutorials/harness-long-task.md)
- [Case task library](../examples/case-tasks/index.md)
- [Release readiness audit task](../examples/case-tasks/release-readiness-audit.md)

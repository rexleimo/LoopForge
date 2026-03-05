# 5-Minute Outcomes

This page answers one question fast:
what can LoopForge actually produce for me today?

## Prerequisites

```bash
ollama serve
loopforge init
```

## Outcome 1: First artifact in a new workspace

```bash
mkdir -p demo-work
loopforge agent run --workspace demo-work --prompt "Create notes/hello.md with a short intro to this workspace and 3 next actions."
```

Expected artifact:
- `demo-work/notes/hello.md`

## Outcome 2: Fix one failing test with a report

```bash
loopforge agent run --workspace . --prompt "Run tests. Fix one failing test. Re-run that test. Write notes/fix-report.md with root cause and patch summary."
```

Expected artifact:
- `notes/fix-report.md`

## Outcome 3: Release readiness memo

```bash
loopforge agent run --workspace . --prompt "Read CHANGELOG.md and recent commits. Create notes/release-readiness.md with: risks, blockers, go/no-go recommendation, and next actions."
```

Expected artifact:
- `notes/release-readiness.md`

## Why this matters

These are not chat replies only. They are persistent outputs you can review, commit, and hand over.

Next:
- [Case task library](../examples/case-tasks/index.md)
- [Why LoopForge](../explanation/why-loopforge.md)

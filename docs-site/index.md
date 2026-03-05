<div class="rexos-hero" markdown>

# LoopForge

**Your Personal AI Engineer for real shipping work.**

From prompt to deliverable artifacts: fix report, release checklist, research memo, and reproducible checkpoints.

[Start in 5 minutes](tutorials/five-minute-outcomes.md){ .md-button .md-button--primary }
[What it can do](examples/case-tasks/index.md){ .md-button }
[Why LoopForge](explanation/why-loopforge.md){ .md-button }
[Quick intro](blog/what-is-loopforge.md){ .md-button }
[Personal AI Engineer](blog/personal-ai-engineer.md){ .md-button }

<p class="rexos-muted">
OpenClaw is known as a personal assistant. LoopForge is positioned as a personal AI engineer for builders: local-first, reproducible, and audit-friendly.
</p>

</div>

> Brand update: LoopForge is the new name (formerly RexOS). CLI is `loopforge`; config stays in `~/.rexos`.

<div class="grid cards" markdown>

- :material-hammer-wrench: **Fix One Failing Test**
  Ask LoopForge to run tests, repair one failure, and write `notes/fix-report.md`.
  [Copy/paste prompt](examples/case-tasks/fix-one-failing-test.md)

- :material-clipboard-check: **Release Readiness Audit**
  Generate a practical release go/no-go checklist from commits, tests, and changelog.
  [Copy/paste prompt](examples/case-tasks/release-readiness-audit.md)

- :material-file-document-edit: **Routing Plan + Cost Notes**
  Produce provider/model routing guidance with trade-offs and rollback notes.
  [Copy/paste prompt](examples/case-tasks/provider-routing-plan.md)

- :material-history: **Reproducible Progress**
  Keep a clear trail: change -> verify -> checkpoint.
  [Harness workflow](tutorials/harness-long-task.md)

</div>

## 3 Fast Outcomes

=== "1) First successful run"
    ```bash
    ollama serve
    loopforge init
    mkdir -p my-work
    loopforge agent run --workspace my-work --prompt "Create notes/hello.md with a short project intro."
    ```

=== "2) Fix one failing test"
    ```bash
    loopforge agent run --workspace . --prompt "Run tests. Fix one failing test. Re-run that test. Write notes/fix-report.md with root cause and patch summary."
    ```

=== "3) Release audit memo"
    ```bash
    loopforge agent run --workspace . --prompt "Read CHANGELOG.md and recent commits. Create notes/release-readiness.md with: risks, blockers, go/no-go, and next actions."
    ```

## Where We Fit

- Choose **LoopForge** when your primary need is engineering delivery and reproducible execution.
- Choose assistant-style products when your primary need is daily life/chat experience.
- Choose broad operation platforms when your primary need is channel coverage first.

See detailed positioning: [Why LoopForge](explanation/why-loopforge.md).

## Next Steps

- [5-minute outcomes](tutorials/five-minute-outcomes.md)
- [Personal AI Engineer narrative](blog/personal-ai-engineer.md)
- [Case task library](examples/case-tasks/index.md)
- [Providers & routing](how-to/providers.md)

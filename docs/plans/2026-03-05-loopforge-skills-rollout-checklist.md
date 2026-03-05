# LoopForge Skills Rollout Checklist

## Scope

- Skills manifest/schema validation
- Skills discovery and dependency resolver
- Runtime skill policy and approval gate
- Skill lifecycle events and audit records
- CLI command surface: `loopforge skills list|show|doctor|run`
- Docs and quickstart (EN + ZH)

## Release Gates

1. `cargo test -p rexos-skills`
2. `cargo test -p rexos --test runtime_skills_policy`
3. `cargo test -p rexos --test skills_audit_events`
4. `cargo test -p loopforge-cli`
5. `cargo test --workspace --locked`
6. `python3 -m mkdocs build --strict`

All gates must pass before merge/release.

## Rollout Steps

1. Release with `[skills]` table documented as default-off optional policy.
2. Announce CLI commands and local-skill directory precedence.
3. Collect first-week telemetry from ACP events and `rexos.audit.skill_runs`.
4. Enable stricter approval policy in internal environments before broader rollout.

## Monitoring

Track the following signals:

- `skill.blocked` rate by reason (`session_whitelist`, `policy_allowlist`, `approval_required`)
- `skill.failed` top causes
- Ratio of `skill.executed / skill.discovered`
- User-reported false positives in approval policy

## Rollback Conditions

Rollback or hotfix immediately if any condition is met:

- `loopforge skills run` cannot execute baseline readonly skills in supported environments
- `skill.blocked` spikes due to policy misconfiguration affecting normal workflows
- Runtime regression causes unrelated `agent run` flows to fail
- Workspace tests pass locally but fail consistently in CI after skills changes

## Rollback Actions

1. Revert skills CLI command routing from `main.rs`.
2. Disable policy enforcement by clearing session skill policy and unsetting `require_approval`.
3. Keep audit/event writes enabled for incident diagnosis.
4. Publish short incident note + expected recovery ETA.

## Follow-up (Marketplace-ready)

- Skill packaging/signing format
- Trusted source verification
- Remote distribution/index service
- Versioned migration tooling for skill APIs

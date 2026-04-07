# Skills Reference

LoopForge supports a local Skills framework for reusable workflows.

## Skill Directory Priority

LoopForge discovers skills in this precedence order (later wins on name conflict):

1. `~/.codex/skills` (`home`)
2. `<workspace>/.loopforge/skills` (`workspace`)

## Skill Manifest (`skill.toml`)

```toml
name = "hello-skill"
version = "0.1.0"
entry = "SKILL.md"
permissions = ["readonly", "tool:fs_read"]

[[dependencies]]
name = "shared-style"
version_req = "^1"
```

Required fields:

- `name`
- `version`
- `entry`

Optional fields:

- `permissions`
- `dependencies`

## CLI Commands

### List skills

```bash
loopforge skills list --workspace .
```

### Show one skill

```bash
loopforge skills show hello-skill --workspace .
```

### Doctor check

```bash
loopforge skills doctor --workspace .
loopforge skills doctor --workspace . --strict
```

### Install one skill from a remote archive

```bash
loopforge skills install https://example.com/hello-skill.zip --workspace .
```

Useful flags:

- `--format <auto|zip|tar|tar-gz>` (default `auto`)
- `--force` to replace an existing installed skill with the same manifest name
- `--json` for machine-readable output

Security guards on install:

- archive extraction is pinned to a canonical install root (`<workspace>/.loopforge/skills`)
- parent traversal (`../`) and absolute paths are rejected
- symlink and hardlink archive entries are rejected

### Run a skill

```bash
loopforge skills run hello-skill --workspace . --input "Summarize README"
```

## Policy and Approval

`~/.loopforge/config.toml` supports a `[skills]` table:

```toml
[skills]
allowlist = ["hello-skill", "qa-helper"]
require_approval = true
auto_approve_readonly = true
experimental = true
```

Approval env var for non-readonly skills:

```bash
export LOOPFORGE_SKILL_APPROVAL_ALLOW=hello-skill
# or
export LOOPFORGE_SKILL_APPROVAL_ALLOW=all
```

## Events and Audit

Runtime writes ACP events for skill lifecycle:

- `skill.discovered`
- `skill.loaded`
- `skill.blocked`
- `skill.executed`
- `skill.failed`

Skill audit records are stored at key `rexos.audit.skill_runs`.

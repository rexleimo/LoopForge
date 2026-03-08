# Skills Quickstart

## What success looks like

By the end, your skill should be discoverable by `loopforge skills list`, pass `loopforge skills doctor`, and run once with a real output.

This tutorial helps beginners create and run a first skill in under 10 minutes.

## 1. Create Skill Folder

```bash
mkdir -p .loopforge/skills/hello-skill
```

## 2. Add `skill.toml`

Create `.loopforge/skills/hello-skill/skill.toml`:

```toml
name = "hello-skill"
version = "0.1.0"
entry = "SKILL.md"
permissions = ["readonly", "tool:fs_read"]
```

## 3. Add `SKILL.md`

Create `.loopforge/skills/hello-skill/SKILL.md`:

```md
# hello-skill

Read the repository README and output:
1. Project goal
2. Top 3 commands
3. One likely setup risk
```

## 4. Verify Discovery

```bash
loopforge skills list --workspace .
loopforge skills show hello-skill --workspace .
loopforge skills doctor --workspace .
```

## 5. Run It

```bash
loopforge skills run hello-skill --workspace . --input "Analyze this repo"
```

## 6. Optional: Enable approval policy

Edit `~/.loopforge/config.toml`:

```toml
[skills]
allowlist = ["hello-skill"]
require_approval = true
auto_approve_readonly = true
experimental = true
```

For non-readonly permissions, approve explicitly:

```bash
export LOOPFORGE_SKILL_APPROVAL_ALLOW=hello-skill
```

## Common Errors

- `skill not found`: check folder path and `name` in `skill.toml`
- `entry file missing`: verify `entry` points to a real file
- `approval required`: set `LOOPFORGE_SKILL_APPROVAL_ALLOW` or reduce dangerous permissions

# Skills 快速上手

本教程面向新手，10 分钟内完成第一个 skill 的创建和运行。

## 1. 创建目录

```bash
mkdir -p .loopforge/skills/hello-skill
```

## 2. 写 `skill.toml`

创建 `.loopforge/skills/hello-skill/skill.toml`：

```toml
name = "hello-skill"
version = "0.1.0"
entry = "SKILL.md"
permissions = ["readonly", "tool:fs_read"]
```

## 3. 写 `SKILL.md`

创建 `.loopforge/skills/hello-skill/SKILL.md`：

```md
# hello-skill

读取仓库 README 并输出：
1. 项目目标
2. 3 个核心命令
3. 1 个可能的环境风险
```

## 4. 检查发现结果

```bash
loopforge skills list --workspace .
loopforge skills show hello-skill --workspace .
loopforge skills doctor --workspace .
```

## 5. 运行 skill

```bash
loopforge skills run hello-skill --workspace . --input "分析这个仓库"
```

## 6. 可选：开启审批策略

编辑 `~/.loopforge/config.toml`：

```toml
[skills]
allowlist = ["hello-skill"]
require_approval = true
auto_approve_readonly = true
experimental = true
```

如果 skill 有高风险权限，需要显式批准：

```bash
export LOOPFORGE_SKILL_APPROVAL_ALLOW=hello-skill
```

## 常见报错

- `skill not found`：检查目录路径和 `skill.toml` 的 `name`
- `entry file missing`：检查 `entry` 指向的文件是否存在
- `approval required`：设置 `LOOPFORGE_SKILL_APPROVAL_ALLOW` 或降低危险权限

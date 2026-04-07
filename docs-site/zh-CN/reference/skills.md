# Skills 参考

LoopForge 支持本地 Skills 框架，用于把可复用流程沉淀成稳定能力。

## Skills 目录优先级

同名 skill 采用后者覆盖前者：

1. `~/.codex/skills`（`home`）
2. `<workspace>/.loopforge/skills`（`workspace`）

## Manifest（`skill.toml`）

```toml
name = "hello-skill"
version = "0.1.0"
entry = "SKILL.md"
permissions = ["readonly", "tool:fs_read"]

[[dependencies]]
name = "shared-style"
version_req = "^1"
```

必填项：

- `name`
- `version`
- `entry`

可选项：

- `permissions`
- `dependencies`

## CLI 命令

### 列出 skills

```bash
loopforge skills list --workspace .
```

### 查看单个 skill

```bash
loopforge skills show hello-skill --workspace .
```

### Doctor 检查

```bash
loopforge skills doctor --workspace .
loopforge skills doctor --workspace . --strict
```

### 从远程归档安装一个 skill

```bash
loopforge skills install https://example.com/hello-skill.zip --workspace .
```

常用参数：

- `--format <auto|zip|tar|tar-gz>`（默认 `auto`）
- `--force`：当同名 skill 已存在时覆盖安装
- `--json`：输出机器可读 JSON

安装过程的安全护栏：

- 解压根目录固定在 canonical install root（`<workspace>/.loopforge/skills`）
- 拒绝 `../` 路径穿越与绝对路径
- 拒绝 symlink / hardlink 归档条目

### 运行 skill

```bash
loopforge skills run hello-skill --workspace . --input "总结 README"
```

## 策略与审批

`~/.loopforge/config.toml` 中支持 `[skills]`：

```toml
[skills]
allowlist = ["hello-skill", "qa-helper"]
require_approval = true
auto_approve_readonly = true
experimental = true
```

非只读 skill 可通过环境变量批准：

```bash
export LOOPFORGE_SKILL_APPROVAL_ALLOW=hello-skill
# 或
export LOOPFORGE_SKILL_APPROVAL_ALLOW=all
```

## 事件与审计

运行时会写入 skill 生命周期 ACP 事件：

- `skill.discovered`
- `skill.loaded`
- `skill.blocked`
- `skill.executed`
- `skill.failed`

skill 审计记录存储在 `rexos.audit.skill_runs`。

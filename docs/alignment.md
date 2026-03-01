# RexOS 对齐清单（Anthropic Harness + OpenFang 架构参考）

本文档用于“可审计地”回答：RexOS 是否已按
- Anthropic: *Effective harnesses for long-running agents*
- OpenFang: `openfang/docs/architecture.md`

对齐到可用的 Agent OS 核心能力与扩展点。

> 说明：OpenFang 的完整功能面非常大（channels/skills/wire/MCP/A2A/WASM/计费/向量检索等）。RexOS 当前目标是对齐其 **核心循环 + 工程化 harness + 安全工具模型 + 多 provider LLM**，并在文档中明确哪些属于后续子系统。

---

## 1) Anthropic “Long-running Harness” 对齐

### 1.1 Durable artifacts（可持久化产物）

- `features.json`：存在（workspace 内），作为可持续 checklist  
  - 创建：`rexos harness init <dir>`（无 `--prompt`）或 `rexos harness init <dir> --prompt ...`（初始化并生成内容）
  - 规则：在 system prompt 中强化；文件内仍保留规则字段（后续可进一步增强 schema 校验）
- `rexos-progress.md`：存在（workspace 内），append-only 日志
- `init.sh`：存在（workspace 内），作为 smoke/test 入口
- Git commit checkpoint：存在（workspace 内的 git repo），harness 会在需要时自动 checkpoint

对应实现：
- `crates/rexos-harness/src/lib.rs`：`init_workspace()`, `bootstrap_with_prompt()`, `run_harness()`

### 1.2 Initializer agent（“初始化阶段”）

目标：把用户 prompt 扩展成完整 `features.json` 并验证 `init.sh` 可跑通，然后 checkpoint。

对应实现：
- `rexos harness init <dir> --prompt "<prompt>"`  
  - 内部调用：`rexos::harness::bootstrap_with_prompt()`
  - 行为：preflight → 运行 initializer system prompt → 运行 `init.sh` → git checkpoint

测试覆盖：
- `crates/rexos/tests/harness_initializer.rs`

### 1.3 Coding harness（“长任务执行阶段”）

目标：每次只推进小步（一个 feature/一个子目标），并且在每次 agent 输出后：
- 运行 `init.sh` 进行验证
- 失败则将失败输出喂回 agent，重试（有上限）
- 成功则自动 checkpoint commit（若工作区有变更）

对应实现：
- `rexos harness run <dir> --prompt "<prompt>" --max-attempts 3`  
  - 默认 session：workspace 级持久化（见 1.4）
  - 内部调用：`rexos::harness::run_harness()`

测试覆盖：
- `crates/rexos/tests/harness_runner.rs`

### 1.4 Session persistence（跨多次运行续跑）

目标：同一 workspace 默认复用同一个 session id（避免每次 run 都是新 session）。

对应实现：
- `crates/rexos-harness/src/lib.rs`：`resolve_session_id()` → `workspace/.rexos/session_id`
- CLI 默认行为：`rexos harness run` / `rexos harness init --prompt` 在未显式传 `--session` 时使用 `resolve_session_id()`

测试覆盖：
- `crates/rexos/tests/harness_session_persistence.rs`

---

## 2) OpenFang 架构参考对齐（核心层）

### 2.1 Workspace crate 分层（OpenFang-style）

已对齐到 workspace + 多 crate 的基础形态：
- `crates/rexos`（facade，对外 re-export，保持 `rexos::agent/llm/memory/tools/...` 路径稳定）
- `crates/rexos-cli`（二进制 `rexos`）
- `crates/rexos-kernel`（当前为 config/paths/router 等“内核基础类型”，后续可扩展为真正 kernel 组装器）
- `crates/rexos-runtime`（agent loop）
- `crates/rexos-memory`（SQLite 记忆）
- `crates/rexos-llm`（drivers + router/registry）
- `crates/rexos-tools`（工具沙盒）
- `crates/rexos-harness`（长任务 harness）
- `crates/rexos-daemon`（HTTP daemon 骨架）

### 2.2 LLM driver abstraction + 多 provider

已实现：
- OpenAI-compatible driver（可用于 Ollama、小模型、以及多数网关）
- Provider-native：DashScope、Zhipu(GLM)、MiniMax、Anthropic、Gemini
- Router：按 `TaskKind` 选择 `(provider, model)`；支持 `model="default"` 走 provider 的 `default_model`

对应实现：
- `crates/rexos-llm/src/*`
- `crates/rexos-kernel/src/config.rs`（providers/router）

### 2.3 Memory substrate（最小可用）

已实现（MVP）：
- SQLite：sessions/messages + kv
- tool_calls_json 持久化（用于复盘与审计）

对应实现：
- `crates/rexos-memory/src/lib.rs`

未实现（后续子系统）：
- embeddings / semantic search / usage metering / knowledge graph / canonical sessions

### 2.4 Capability-based 工具模型（MVP）

已实现（MVP）：
- 工具沙盒：workspace 路径约束、防 `..`、拒绝 symlink escape
- shell：固定 cwd + env_clear + timeout + 基础危险命令拒绝

新增（参考 OpenFang）：
- `web_fetch`：SSRF 防护（默认拒绝 loopback/private/link-local），支持 `allow_private=true`（便于本地测试/内网）

对应实现：
- `crates/rexos-tools/src/lib.rs`：`fs_read/fs_write/shell/web_fetch`
- 测试：`crates/rexos/tests/tools_web_fetch.rs`

未实现（后续子系统）：
- 细粒度 capability grant（按 agent / session / role 动态过滤工具）
- MCP/A2A 外部工具命名空间
- WASM sandbox / fuel+epoch metering

### 2.5 Agent loop stability（稳定性子系统）

新增（参考 OpenFang LoopGuard）：
- Tool-loop guard：同一 session 内重复调用相同工具（同参）达到阈值即 fail-fast（避免无意义循环）

对应实现：
- `crates/rexos-runtime/src/lib.rs`
- 测试：`crates/rexos/tests/loop_guard.rs`

未实现（后续子系统）：
- session repair / compaction / 审计链（Merkle/hash chain）

---

## 3) 仍未对齐的 OpenFang 大子系统（明确列出）

以下属于后续阶段（RexOS 需要做“骨架 + 渐进实现”）：
- Kernel 组装器（真正的 subsystem registry / scheduler / supervisor / event bus）
- API server（丰富的 REST/WS/SSE；OpenAI-compatible endpoints；A2A endpoints）
- Channels system（多平台适配）
- Skills system（skill registry + marketplace）
- Wire protocol（p2p + auth）
- Web search（多 provider 搜索聚合）
- Metering/quotas/cost catalog
- Embeddings/semantic memory


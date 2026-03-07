# 发布就绪审计

**目标：** 在打 tag 之前产出可执行的发布检查清单与阻塞项。

## 运行

1) 先 `cd` 到目标仓库。

2) 执行：

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "对这个仓库做发布就绪审计。检查 Cargo.toml（或其他包版本文件）、CHANGELOG.md、.github/workflows 下的发布工作流、以及 scripts/package_release.py（若存在）。写 notes/release-readiness-audit.md，包含：1) 版本/Tag 一致性 2) Changelog 就绪度 3) CI/发布流程就绪度 4) 打包产物命名检查 5) 阻塞项（P0/P1） 6) 推荐发布命令顺序。要求偏实操、可执行。"
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "对这个仓库做发布就绪审计。检查 Cargo.toml（或其他包版本文件）、CHANGELOG.md、.github/workflows 下的发布工作流、以及 scripts/package_release.py（若存在）。写 notes/release-readiness-audit.md，包含：1) 版本/Tag 一致性 2) Changelog 就绪度 3) CI/发布流程就绪度 4) 打包产物命名检查 5) 阻塞项（P0/P1） 6) 推荐发布命令顺序。要求偏实操、可执行。"
    ```

## 预期产物

- `notes/release-readiness-audit.md`

!!! note
    这是发布前分析任务，不应创建 tag 或实际发布。在 LoopForge 自身仓库里，维护者现在通常只需要把版本号与 `CHANGELOG.md` 的更新合并进 `main`，后续由 Actions 自动创建缺失的 semver tag，再由现有 Release 工作流发布 GitHub 版本。

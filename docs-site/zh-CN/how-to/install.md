# 安装与更新

## 推荐路径

对大多数用户，最稳妥的顺序是：

1. 安装 `loopforge`
2. 运行 `loopforge --help`
3. 运行 `loopforge init`
4. 运行 `loopforge config validate`
5. 运行 `loopforge doctor`

## 方案 A：下载预编译二进制（推荐）

1. 从 GitHub Releases 下载你系统对应的压缩包。
2. 解压。
3. 把 `loopforge`（或 `loopforge.exe`）放到 `PATH` 里。

然后运行：

```bash
loopforge --help
loopforge init
```

## 方案 B：从源码安装（Cargo）

```bash
cargo install --path crates/loopforge-cli --locked
loopforge --help
```

## 更新

- 如果通过 Releases 安装：下载更新版本的压缩包并替换旧二进制。
- 如果通过 Cargo 安装：重新执行 `cargo install --path crates/loopforge-cli --locked`。

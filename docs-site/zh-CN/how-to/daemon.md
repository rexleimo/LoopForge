# 运行 Daemon

## 什么时候看这页

当你想把 LoopForge 作为一个小型 HTTP 服务运行，并且需要快速健康检查路径时，先看这页。

LoopForge 内置一个 HTTP daemon（目前功能最小化）。

## 启动

```bash
loopforge daemon start --addr 127.0.0.1:8787
```

## 健康检查

```bash
curl http://127.0.0.1:8787/healthz
```

预期返回：

```json
{ "status": "ok" }
```


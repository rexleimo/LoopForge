# Run the Daemon

## Use this page when

Read this page when you want to run LoopForge as a small HTTP service and need a fast health-check path.

LoopForge includes an HTTP daemon (currently minimal).

## Start

```bash
loopforge daemon start --addr 127.0.0.1:8787
```

## Health check

```bash
curl http://127.0.0.1:8787/healthz
```

Expected response:

```json
{ "status": "ok" }
```


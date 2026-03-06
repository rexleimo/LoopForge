# 什么是 LoopForge？

LoopForge是一个本地优先的长任务 Agent OS。

## 一句话定位

LoopForge = **本地先跑通 + 跑通了再切强模型**。

不是「咔咔一顿输出最后全错了」，而是「改一下 → 跑一下 → 对了再继续」。

## 给谁用的

- 想要可复现的编程循环，而不是一次性的聊天输出
- 需要持久化 checkpoint 和产物记录
- 本地用 Ollama 跑通后，想切到更强的云模型

## 改了啥

- 名称：LoopForge
- CLI：`loopforge`
- 配置目录：还是 `~/.loopforge`（不变）

## 3 行跑起来

```bash
ollama serve
loopforge init
loopforge agent run --workspace demo --prompt "Create hello.txt with hi"
```

## 相关链接

- [快速开始](../tutorials/quickstart-ollama.md)
- [Provider 配置](../how-to/providers.md)
- [Harness 教程](../tutorials/harness-long-task.md)

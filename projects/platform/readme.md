# platform

运行环境适配区。

```text
web      → Browser APIs（无 Rust）
desktop  → IPC → host/bridge
mobile   → IPC → host/bridge
wasm     → 显式 opt-in 能力（非默认 client 后端）
```

# examples — 按用途与运行形态分类

```text
frontend/      前端客户端运行时
fullstack/     client + server
backend/       无 WAE 前端的后端（typescript/ · rust/ 占位）
native/        Rust 壳 · WebView · IPC · 系统能力
integration/   与 Vue/React/Svelte/Solid 融合（框架是变量，不是拓扑）
minimal/       最小依赖与单一能力边界
```

原则：WAE 核心按能力组织；examples 按用途组织；框架作为集成变量；后端语言作为实现变量。

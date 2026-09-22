# `@wae/wae-unknown-wasm32`

一等 Wasm 客户端 runtime 入口（最小宿主假设）。默认浏览器路径不会自动启用本包。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-unknown-wasm32` @ `0.0.0` |
| 平台 id | `unknown-wasm32` |
| 谁需要 | 显式 Wasm 客户端 |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-unknown-wasm32@0.0.0` |
| os/cpu | 无 os/cpu 限制；须在支持 Wasm 的宿主中使用 |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | wasm32 目标；具体 .wasm 产物尚未捆入本 npm 包。 |
| 不支持 / 未完成 | 无预置 wasm-bindgen 胶水与 .wasm 文件。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-unknown-wasm32";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

pnpm --filter @wae/wae-unknown-wasm32 run build；后续再接 Rust wasm 构建。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../readme.md)
- Host：[host/bridge](../../host/bridge/readme.md)

# `@wae/wae-darwin-x64`

macOS Intel host 入口。Apple Silicon 应使用 darwin-arm64，而非本包。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-darwin-x64` @ `0.0.0` |
| 平台 id | `darwin-x64` |
| 谁需要 | macOS Intel 桌面 WebView |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-darwin-x64@0.0.0` |
| os/cpu | os: darwin · cpu: x64 |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | macOS + WKWebView/同类壳（规划）；无 SDK 安装说明因二进制未发布。 |
| 不支持 / 未完成 | 无 .app / dylib。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-darwin-x64";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

在 Intel Mac 上 build；签名与公证流程未文档化（尚未需要）。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../readme.md)
- Host：[wae-bridge](../../crates/wae-bridge/readme.md)

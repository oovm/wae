# `@wae/wae-win32-x64`

Windows x64 原生 host 入口 +（规划中）嵌入 Wasm。由 @wae/wae optionalDependencies 在匹配机器上安装。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-win32-x64` @ `0.0.0` |
| 平台 id | `win32-x64` |
| 谁需要 | Windows 桌面 WebView |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-win32-x64@0.0.0` |
| os/cpu | os: win32 · cpu: x64 |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | 目标：随包分发 native 二进制并由 platform.start/run 加载。0.0.0 仅有 TS 壳。 |
| 不支持 / 未完成 | 无 .exe / .dll / .node；CLI 尚未调用本包。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-win32-x64";
console.log(platform.id); // "win32-x64"
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

在 Windows x64 上 pnpm --filter @wae/wae-win32-x64 run build；原生链接步骤未接线。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../readme.md)
- Host：[wae-bridge](../../crates/wae-bridge/readme.md)

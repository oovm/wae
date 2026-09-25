# `@wae/wae-win32-arm64`

与 win32-x64 同角色，架构为 arm64。不要在 x64 机器上强装本包作为运行时。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-win32-arm64` @ `0.0.0` |
| 平台 id | `win32-arm64` |
| 谁需要 | Windows ARM64 桌面 WebView |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-win32-arm64@0.0.0` |
| os/cpu | os: win32 · cpu: arm64 |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | Windows ARM64；原生二进制未捆入。 |
| 不支持 / 未完成 | 同其它桌面包：无预编译 host。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-win32-arm64";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

需在 win32-arm64 或交叉环境构建原生产物（未提供脚本）。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../readme.md)
- Host：[wae-bridge](../../crates/wae-bridge/readme.md)

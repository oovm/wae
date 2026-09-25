# `@wae/wae-ios-arm64`

iOS arm64 移动壳入口。与 macOS darwin-arm64 桌面包不同。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-ios-arm64` @ `0.0.0` |
| 平台 id | `ios-arm64` |
| 谁需要 | iOS WebView 壳 |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-ios-arm64@0.0.0` |
| os/cpu | os: darwin · cpu: arm64（包管理器字段；真机仍是 iOS） |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | iOS；最低版本未公布；需 Xcode。 |
| 不支持 / 未完成 | 无 .ipa / xcframework。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-ios-arm64";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

仅在 macOS + Xcode 环境维护；当前仅 TS 壳。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../readme.md)
- Host：[wae-bridge](../../crates/wae-bridge/readme.md)

# `@wae/wae-android-arm64`

Android arm64 移动壳入口。依赖 Android SDK / NDK 的打包流程尚未接线。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-android-arm64` @ `0.0.0` |
| 平台 id | `android-arm64` |
| 谁需要 | Android WebView 壳 |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-android-arm64@0.0.0` |
| os/cpu | cpu: arm64（无 Node os 字段；勿当作 Node 插件） |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | Android arm64-v8a；最低 API 级别未公布。 |
| 不支持 / 未完成 | 无 APK/AAB；无法用 node 直接 run。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-android-arm64";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

需要 Android Studio / SDK；当前仅 TS 壳。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../../readme.md)
- Host：[host/bridge](../../../../host/bridge/readme.md)

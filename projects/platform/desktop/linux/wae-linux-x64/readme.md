# `@wae/wae-linux-x64`

Linux x64 host 入口。发行版差异（glibc/musl、WebKitGTK）未在 0.0.0 固定。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-linux-x64` @ `0.0.0` |
| 平台 id | `linux-x64` |
| 谁需要 | Linux x64 桌面 WebView |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-linux-x64@0.0.0` |
| os/cpu | os: linux · cpu: x64 |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | Linux x64；需后续声明最低 glibc / WebKit 版本。 |
| 不支持 / 未完成 | 无 ELF 二进制；无 AppImage/Flatpak 说明。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-linux-x64";
console.log(platform.id);
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

在目标发行版上编译；CI 矩阵未覆盖全部 distro。

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../../../readme.md)
- Host：[host/bridge](../../../../host/bridge/readme.md)

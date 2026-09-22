# platform

给**平台包维护者**与需要判断「该装哪个 `@wae/wae-*`」的人。

应用作者一般**不要**把平台包写成普通 `dependencies`：安装 `@wae/wae` 后，平台包作为 `optionalDependencies` 按 `os` / `cpu` 拉取。

## 公开包

| npm | 目标 | 备注 |
|-----|------|------|
| `@wae/wae-web` | 浏览器 / PWA | 无 `os`/`cpu` 限制 |
| `@wae/wae-unknown-wasm32` | Wasm 客户端 | 显式 opt-in |
| `@wae/wae-win32-x64` / `win32-arm64` | Windows | `os`+`cpu` |
| `@wae/wae-darwin-x64` / `darwin-arm64` | macOS | `os`+`cpu` |
| `@wae/wae-linux-x64` / `linux-arm64` | Linux | `os`+`cpu` |
| `@wae/wae-android-arm64` | Android | `cpu: arm64` |
| `@wae/wae-ios-arm64` | iOS | `os: darwin` + `cpu: arm64`（打包机约束） |

每个包导出同一形状的 `platform`：`id`、`start` / `build` / `run`（**0.0.0 为空实现**），**尚未**内嵌可执行 native 二进制。

## 用户 vs 维护者

- **应用用户**：确认本机 optional 是否装上；用 `defineConfig({ platform: { client } })` 选 id；不要手抄内部路径。
- **维护者**：在本目录改对应包、`tsc` 产出 `dist`、核对 `package.json` 的 `wae.platform` 与 `os`/`cpu`；与 [`../host`](../host/readme.md) 的加载约定对齐后再谈打二进制。

验证加载（占位阶段）：`import platform from "@wae/wae-web"`（或本机对应包），确认 `platform.id` 字符串正确即可。

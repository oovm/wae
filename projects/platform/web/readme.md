# `@wae/wae-web`

纯 TS 平台入口；使用 Browser APIs；不加载 Rust / 原生二进制。

## 给应用用户

| 项 | 内容 |
|----|------|
| npm 包 | `@wae/wae-web` @ `0.0.0` |
| 平台 id | `web` |
| 谁需要 | 浏览器 / PWA |
| 安装 | 优先安装 `@wae/wae`，由 `optionalDependencies` 按机器拉取；也可显式 `pnpm add @wae/wae-web@0.0.0` |
| os/cpu | 无 os/cpu 限制（任意环境可装） |
| 产物 | `dist/index.js` + 类型声明（TypeScript 壳） |
| 运行前提 | 现代浏览器（需 Fetch / ES modules）。无 Node ABI。 |
| 不支持 / 未完成 | 不提供窗口、文件系统、系统菜单。那些属于桌面/移动 host。 |
| 与 host 关系 | 桌面/移动：经 Rust `host/bridge` IPC；web：无 host；wasm：显式 Wasm 路径 |

验证加载（壳层）：

```ts
import platform from "@wae/wae-web";
console.log(platform.id); // "web"
```

`start` / `build` / `run` 在 0.0.0 为 no-op，成功仅表示 API 可调用，不表示窗口已打开。

## 给维护者

pnpm --filter @wae/wae-web run build → dist/

不要把应用安装步骤与原生交叉编译混在同一段「快速开始」里。

## 相关

- 总览：[platform 区](../readme.md)
- Host：[host/bridge](../../host/bridge/readme.md)

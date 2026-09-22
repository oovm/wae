# frontend adapters

四个包各自独立安装，**不要**指望一个万能 adapter。

| 包 | 注入模型 | 配置工厂 |
|----|----------|----------|
| [`@wae/adapter-vue`](vue/readme.md) | 应用 `provide` / `inject` | `vue()` |
| [`@wae/adapter-react`](react/readme.md) | `WaeProvider` / `useWae` | `react()` |
| [`@wae/adapter-svelte`](svelte/readme.md) | `setWaeContext` / `getWaeContext` | `svelte()` |
| [`@wae/adapter-solid`](solid/readme.md) | Solid context Provider | `solid()` |

共同点：都依赖 `@wae/client`；都**不**提供 UI 组件。  
没有 `@wae/adapter-vanilla`——纯 TS/DOM 直接 `createClient`。

0.0.0：各包的 context 读写多为骨架（抛错或 no-op）；`*()` 工厂已可供 `defineConfig` 使用。

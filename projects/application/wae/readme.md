# @wae/wae

唯一 CLI：识别框架、注入 runtime / platform / server、选择原生壳、编排 dev/build。

不替代 Vite / Nuxt / Next / SvelteKit 等框架 CLI。

```ts
import { defineConfig } from "@wae/wae";

export default defineConfig({
  frontend: { framework: "react" },
  server: { entry: "./server/index.ts" },
  target: "desktop",
});
```

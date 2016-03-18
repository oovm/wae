# @wae/client

框架无关的 TypeScript 应用运行时。纯 TS / 原生 DOM **直接依赖本包**，无需 vanilla adapter。

```ts
import { createClient } from "@wae/client";

const client = createClient({
  server: { baseUrl: "/api" },
});
```

Vue / React / Svelte / Solid 使用独立 `@wae/adapter-*`。

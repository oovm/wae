import { defineConfig } from "@wae/wae";

export default defineConfig({
  client: { entry: "./src/client/main.tsx" },
  server: { entry: "./src/server/main.ts", adapter: "node" },
});

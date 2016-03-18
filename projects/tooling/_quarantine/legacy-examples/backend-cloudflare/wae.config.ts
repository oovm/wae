import { defineConfig } from "@wae/wae";

export default defineConfig({
  server: {
    entry: "./src/main.ts",
    adapter: "cloudflare",
  },
});

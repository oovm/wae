import vue from "@wae/adapter-vue";
import { defineConfig } from "@wae/wae";

export default defineConfig({
    frontend: {
        framework: "vue",
        adapter: vue(),
        entry: "./src/main.ts",
        bundler: "vite",
    },
    target: "desktop",
    platform: { client: "win32-x64" },
});

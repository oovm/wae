import react from "@wae/adapter-react";
import { defineConfig } from "@wae/wae";

export default defineConfig({
    frontend: {
        framework: "react",
        adapter: react(),
        entry: "./src/main.tsx",
    },
    target: "web",
    platform: { client: "web" },
});

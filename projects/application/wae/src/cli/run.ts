/** `wae run` / `wae dev`：按平台启动应用。web 走 Vite。 */

import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import type { ClientPlatformId, FrontendFramework, WaeConfig } from "../index.js";
import { loadWaeConfig } from "./load-config.js";

export type RunMode = "run" | "dev";

type FlagMap = {
    platform?: string;
    port?: number;
    host?: string;
    /** 默认打开浏览器；`--no-open` 关闭 */
    open: boolean;
};

function parseFlags(args: string[]): FlagMap {
    const out: FlagMap = { open: true };
    for (let i = 0; i < args.length; i++) {
        const a = args[i];
        if (a === "--platform" || a === "-p") {
            out.platform = args[++i];
        } else if (a === "--port") {
            out.port = Number(args[++i]);
        } else if (a === "--host") {
            out.host = args[++i];
        } else if (a === "--open") {
            out.open = true;
        } else if (a === "--no-open") {
            out.open = false;
        } else if (typeof a === "string" && a.startsWith("--platform=")) {
            out.platform = a.slice("--platform=".length);
        } else if (typeof a === "string" && a.startsWith("--port=")) {
            out.port = Number(a.slice("--port=".length));
        } else if (typeof a === "string" && a.startsWith("--host=")) {
            out.host = a.slice("--host=".length);
        }
    }
    return out;
}

function resolvePlatformId(flags: FlagMap, config: WaeConfig): ClientPlatformId {
    if (flags.platform) return flags.platform as ClientPlatformId;
    if (config.platform?.client) return config.platform.client;
    if (config.target === "desktop") return "win32-x64";
    if (config.target === "mobile") return "android-arm64";
    return "web";
}

function platformPackageName(id: ClientPlatformId): string {
    return `@wae/wae-${id}`;
}

async function resolveVite(cwd: string) {
    try {
        return await import("vite");
    } catch {
        const fromCwd = pathToFileURL(path.join(cwd, "node_modules", "vite", "dist", "node", "index.js")).href;
        try {
            return await import(fromCwd);
        } catch {
            throw new Error(
                "未找到 `vite`。请在工程中安装：pnpm add -D vite@^7，或在仓库根保证 workspace 已安装。",
            );
        }
    }
}

async function loadFrameworkPlugins(framework: FrontendFramework, cwd: string): Promise<unknown[]> {
    if (framework === "none") return [];

    const tryImport = async (spec: string) => {
        try {
            return await import(spec);
        } catch {
            const local = path.join(cwd, "node_modules", spec);
            if (fs.existsSync(local)) {
                return await import(pathToFileURL(path.join(local, "dist", "index.js")).href);
            }
            // package exports vary — try package root via createRequire-style resolve from cwd
            const { createRequire } = await import("node:module");
            const req = createRequire(path.join(cwd, "package.json"));
            const resolved = req.resolve(spec);
            return await import(pathToFileURL(resolved).href);
        }
    };

    if (framework === "vue") {
        const mod = await tryImport("@vitejs/plugin-vue");
        const plugin = mod.default ?? mod;
        return [typeof plugin === "function" ? plugin() : plugin];
    }
    if (framework === "react") {
        const mod = await tryImport("@vitejs/plugin-react");
        const plugin = mod.default ?? mod;
        return [typeof plugin === "function" ? plugin() : plugin];
    }
    if (framework === "svelte") {
        const mod = await tryImport("@sveltejs/vite-plugin-svelte");
        const plugin = mod.svelte ?? mod.default ?? mod;
        return [typeof plugin === "function" ? plugin() : plugin];
    }
    if (framework === "solid") {
        const mod = await tryImport("vite-plugin-solid");
        const plugin = mod.default ?? mod;
        return [typeof plugin === "function" ? plugin() : plugin];
    }
    return [];
}

async function runWeb(
    cwd: string,
    config: WaeConfig,
    configPath: string,
    flags: FlagMap,
): Promise<void> {
    const vite = await resolveVite(cwd);
    const framework = config.frontend?.framework ?? "none";
    const hasViteConfig = ["vite.config.ts", "vite.config.mts", "vite.config.js", "vite.config.mjs"].some((n) =>
        fs.existsSync(path.join(cwd, n)),
    );

    const plugins = hasViteConfig ? undefined : await loadFrameworkPlugins(framework, cwd);

    const server = await vite.createServer({
        root: cwd,
        configFile: hasViteConfig ? undefined : false,
        plugins: plugins as never,
        server: {
            host: flags.host ?? "127.0.0.1",
            port: flags.port ?? 5173,
            strictPort: false,
            open: flags.open,
        },
        clearScreen: false,
    });

    await server.listen();
    const urls = server.resolvedUrls;
    console.log(`[wae] platform=web framework=${framework}`);
    console.log(`[wae] config=${path.relative(cwd, configPath) || path.basename(configPath)}`);
    if (urls?.local?.length) {
        for (const u of urls.local) console.log(`[wae]  Local:   ${u}`);
    }
    if (urls?.network?.length) {
        for (const u of urls.network) console.log(`[wae]  Network: ${u}`);
    }
    if (!urls?.local?.length) {
        server.printUrls();
    }
    console.log("[wae] 按 Ctrl+C 结束");

    await new Promise<void>((resolve) => {
        const stop = async () => {
            process.off("SIGINT", onSig);
            process.off("SIGTERM", onSig);
            await server.close();
            resolve();
        };
        const onSig = () => {
            void stop();
        };
        process.on("SIGINT", onSig);
        process.on("SIGTERM", onSig);
    });
}

async function runNativePlatform(id: ClientPlatformId, config: WaeConfig): Promise<void> {
    const pkg = platformPackageName(id);
    let mod: { platform?: { run: (o: { entry?: string }) => Promise<void> }; default?: { run: (o: { entry?: string }) => Promise<void> } };
    try {
        mod = await import(pkg);
    } catch (e) {
        throw new Error(
            `无法加载平台包 ${pkg}（${e instanceof Error ? e.message : e}）。请确认已安装 @wae/wae 或其 optionalDependencies。`,
        );
    }
    const platform = mod.platform ?? mod.default;
    if (!platform?.run) {
        throw new Error(`${pkg} 未导出 platform.run`);
    }
    console.log(`[wae] platform=${id} → ${pkg}.run()`);
    await platform.run({ entry: config.frontend?.entry });
    console.log(`[wae] ${pkg}.run() 已返回（0.0.0 原生壳多为空实现）`);
}

export async function cmdRun(args: string[], _opts: { mode: RunMode }): Promise<void> {
    const cwd = process.cwd();
    const flags = parseFlags(args);
    const { path: configPath, config } = await loadWaeConfig(cwd);
    const platformId = resolvePlatformId(flags, config);

    console.log(`[wae] cwd=${cwd}`);
    console.log(`[wae] loaded ${path.relative(cwd, configPath) || path.basename(configPath)}`);

    if (platformId === "web") {
        await runWeb(cwd, config, configPath, flags);
        return;
    }

    await runNativePlatform(platformId, config);
}

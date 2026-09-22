/** `wae run` / `wae dev`：按平台启动应用。web / desktop 默认常用 Vite（可换）。 */

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
    /** web 时默认打开系统浏览器；desktop 忽略 */
    open: boolean;
};

type ViteHandle = {
    // biome-ignore lint/suspicious/noExplicitAny: Vite 类型随 peer 版本变化
    server: any;
    url: string;
    framework: FrontendFramework;
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

function hostDesktopPlatform(): ClientPlatformId {
    const { platform, arch } = process;
    if (platform === "win32") return arch === "arm64" ? "win32-arm64" : "win32-x64";
    if (platform === "darwin") return arch === "arm64" ? "darwin-arm64" : "darwin-x64";
    if (platform === "linux") return arch === "arm64" ? "linux-arm64" : "linux-x64";
    return "win32-x64";
}

function resolvePlatformId(flags: FlagMap, config: WaeConfig): ClientPlatformId {
    if (flags.platform) return flags.platform as ClientPlatformId;
    if (config.platform?.client) return config.platform.client;
    if (config.target === "desktop") return hostDesktopPlatform();
    if (config.target === "mobile") {
        return process.platform === "darwin" ? "ios-arm64" : "android-arm64";
    }
    return "web";
}

function isNativeShellPlatform(id: ClientPlatformId): boolean {
    return id !== "web" && id !== "unknown-wasm32";
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

async function startVite(cwd: string, config: WaeConfig, flags: FlagMap, openBrowser: boolean): Promise<ViteHandle> {
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
            open: openBrowser,
        },
        clearScreen: false,
    });

    await server.listen();
    const local = server.resolvedUrls?.local?.[0];
    if (!local) {
        await server.close();
        throw new Error("Vite 已启动但未得到 Local URL");
    }
    return { server, url: local, framework };
}

async function runWeb(cwd: string, config: WaeConfig, configPath: string, flags: FlagMap): Promise<void> {
    const { server, url, framework } = await startVite(cwd, config, flags, flags.open);
    console.log(`[wae] platform=web framework=${framework}`);
    console.log(`[wae] config=${path.relative(cwd, configPath) || path.basename(configPath)}`);
    console.log(`[wae]  Local:   ${url}`);
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

async function runDesktopShell(
    id: ClientPlatformId,
    cwd: string,
    config: WaeConfig,
    configPath: string,
    flags: FlagMap,
): Promise<void> {
    const bundler = config.frontend?.bundler ?? "vite";
    let vite: ViteHandle | null = null;
    let url = config.frontend?.devUrl;

    if (bundler === "vite") {
        vite = await startVite(cwd, config, flags, false);
        url = vite.url;
        console.log(`[wae] frontend Vite → ${url}`);
    } else if (!url) {
        throw new Error(
            'desktop + bundler:"custom" 需要 frontend.devUrl（例如 http://127.0.0.1:5173/）',
        );
    }

    const pkg = platformPackageName(id);
    let mod: {
        platform?: { run: (o: { entry?: string; url?: string; title?: string }) => Promise<void> };
        default?: { run: (o: { entry?: string; url?: string; title?: string }) => Promise<void> };
    };
    try {
        mod = await import(pkg);
    } catch (e) {
        if (vite) await vite.server.close();
        throw new Error(
            `无法加载平台包 ${pkg}（${e instanceof Error ? e.message : e}）。请确认已安装 @wae/wae 或其 optionalDependencies。`,
        );
    }
    const platform = mod.platform ?? mod.default;
    if (!platform?.run) {
        if (vite) await vite.server.close();
        throw new Error(`${pkg} 未导出 platform.run`);
    }

    console.log(`[wae] platform=${id} → ${pkg}.run({ url })`);
    console.log(`[wae] config=${path.relative(cwd, configPath) || path.basename(configPath)}`);
    console.log("[wae] 关闭桌面窗口后结束");

    try {
        await platform.run({
            entry: config.frontend?.entry,
            url,
            title: `WAE · ${config.frontend?.framework ?? "app"}`,
        });
    } finally {
        if (vite) await vite.server.close();
    }
}

export async function cmdRun(args: string[], _opts: { mode: RunMode }): Promise<void> {
    const cwd = process.cwd();
    const flags = parseFlags(args);
    const { path: configPath, config } = await loadWaeConfig(cwd);
    const platformId = resolvePlatformId(flags, config);

    console.log(`[wae] cwd=${cwd}`);
    console.log(`[wae] loaded ${path.relative(cwd, configPath) || path.basename(configPath)}`);

    if (platformId === "web") {
        const bundler = config.frontend?.bundler ?? "vite";
        if (bundler === "custom") {
            const devUrl = config.frontend?.devUrl;
            console.log("[wae] frontend.bundler=custom：不代启 Vite（可换 Webpack / Rspack 等）");
            if (devUrl) {
                console.log(`[wae] 请自行启动 bundler，开发地址约定为 ${devUrl}`);
            } else {
                console.log(
                    "[wae] 请自行启动 bundler，并在 wae.config 中设置 frontend.devUrl（例如 http://127.0.0.1:3000）",
                );
            }
            return;
        }
        await runWeb(cwd, config, configPath, flags);
        return;
    }

    if (isNativeShellPlatform(platformId)) {
        await runDesktopShell(platformId, cwd, config, configPath, flags);
        return;
    }

    // unknown-wasm32 等：暂只调 platform.run
    const pkg = platformPackageName(platformId);
    const mod = await import(pkg);
    const platform = mod.platform ?? mod.default;
    await platform.run({ entry: config.frontend?.entry });
}

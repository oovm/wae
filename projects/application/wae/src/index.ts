/** @wae/wae — Node CLI 与工程编排（不含 Rust runtime；不替代框架 CLI）。 */

export type ClientPlatformId =
    | "web"
    | "unknown-wasm32"
    | "win32-x64"
    | "win32-arm64"
    | "darwin-x64"
    | "darwin-arm64"
    | "linux-x64"
    | "linux-arm64"
    | "android-arm64"
    | "ios-arm64";

export type ServerAdapterId = "node" | "deno" | "cloudflare" | "bun";

export type FrontendFramework = "vue" | "react" | "svelte" | "solid" | "none";

export type RuntimeTarget = "web" | "desktop" | "mobile";

export type FrontendAdapterFactory = {
    name: string;
};

export type WaeConfig = {
    frontend?: {
        /** 由 CLI 推断或显式指定；`none` = 纯 TS / 无框架 */
        framework?: FrontendFramework;
        /** 可选：adapter 工厂（如 react()），不把包名写进 API */
        adapter?: FrontendAdapterFactory;
        entry?: string;
    };
    server?: {
        entry?: string;
        adapter?: ServerAdapterId;
    };
    /** 运行目标：浏览器 / 桌面壳 / 移动壳 */
    target?: RuntimeTarget;
    platform?: {
        client?: ClientPlatformId;
        server?: ServerAdapterId;
    };
};

/**
 * 配置规范化与类型约束。
 * 文件已是 `wae.config.ts`，API 名无需再带 Wae 前缀。
 */
export function defineConfig(config: WaeConfig): WaeConfig {
    return {
        frontend: {
            framework: config.frontend?.framework ?? "none",
            adapter: config.frontend?.adapter,
            entry: config.frontend?.entry,
        },
        server: config.server,
        target: config.target ?? "web",
        platform: config.platform,
    };
}

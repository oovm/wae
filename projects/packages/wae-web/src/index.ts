/** @wae/wae-web — 浏览器 / PWA 平台包 */

export type StartOptions = {
    entry?: string;
};

export type BuildOptions = {
    outDir?: string;
};

export type RunOptions = {
    entry?: string;
};

export type WaeApp = {
    close(): Promise<void>;
};

export interface WaePlatform {
    readonly id: "web";
    start(options: StartOptions): Promise<WaeApp>;
    build(options: BuildOptions): Promise<void>;
    run(options: RunOptions): Promise<void>;
}

export const platform: WaePlatform = {
    id: "web",
    async start(_options) {
        return {
            async close() {},
        };
    },
    async build(_options) {},
    async run(_options) {},
};

export default platform;

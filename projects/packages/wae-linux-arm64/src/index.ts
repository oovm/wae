/** @wae/wae-linux-arm64 — Linux arm64 native host + embedded WASM */

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
    readonly id: "linux-arm64";
    start(options: StartOptions): Promise<WaeApp>;
    build(options: BuildOptions): Promise<void>;
    run(options: RunOptions): Promise<void>;
}

export const platform: WaePlatform = {
    id: "linux-arm64",
    async start(_options) {
        return {
            async close() {},
        };
    },
    async build(_options) {},
    async run(_options) {},
};

export default platform;

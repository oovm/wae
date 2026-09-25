/** @wae/wae-android-arm64 — Android arm64 native host */

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
    readonly id: "android-arm64";
    start(options: StartOptions): Promise<WaeApp>;
    build(options: BuildOptions): Promise<void>;
    run(options: RunOptions): Promise<void>;
}

export const platform: WaePlatform = {
    id: "android-arm64",
    async start(_options) {
        return {
            async close() {},
        };
    },
    async build(_options) {},
    async run(_options) {},
};

export default platform;

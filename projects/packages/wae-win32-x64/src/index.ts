/** @wae/wae-win32-x64 — Windows x64 native host +（规划）嵌入 WASM */

import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export type StartOptions = {
    entry?: string;
    url?: string;
};

export type BuildOptions = {
    outDir?: string;
};

export type RunOptions = {
    entry?: string;
    /** 前端开发服务器地址（Vite 等） */
    url?: string;
    title?: string;
};

export type WaeApp = {
    close(): Promise<void>;
};

export interface WaePlatform {
    readonly id: "win32-x64";
    start(options: StartOptions): Promise<WaeApp>;
    build(options: BuildOptions): Promise<void>;
    run(options: RunOptions): Promise<void>;
}

function findWorkspaceRoot(start: string): string | null {
    let dir = start;
    for (let i = 0; i < 12; i++) {
        const cargo = path.join(dir, "Cargo.toml");
        if (fs.existsSync(cargo)) {
            const text = fs.readFileSync(cargo, "utf8");
            if (text.includes("wae-desktop") || text.includes("projects/crates/wae-desktop")) {
                return dir;
            }
        }
        const parent = path.dirname(dir);
        if (parent === dir) break;
        dir = parent;
    }
    return null;
}

function resolveDesktopBinary(workspaceRoot: string): { cmd: string; args: string[]; cwd: string } {
    const exe = path.join(
        workspaceRoot,
        "target",
        "debug",
        process.platform === "win32" ? "wae-desktop.exe" : "wae-desktop",
    );
    if (fs.existsSync(exe)) {
        return { cmd: exe, args: [], cwd: workspaceRoot };
    }
    return {
        cmd: "cargo",
        args: ["run", "-p", "wae-desktop", "--"],
        cwd: workspaceRoot,
    };
}

function spawnDesktop(url: string, title: string): Promise<void> {
    const here = path.dirname(fileURLToPath(import.meta.url));
    const root = findWorkspaceRoot(here);
    if (!root) {
        throw new Error(
            "找不到含 `wae-desktop` 的 workspace 根。请在 WAE 仓库内开发，或先 `cargo build -p wae-desktop`。",
        );
    }
    const { cmd, args, cwd } = resolveDesktopBinary(root);
    const fullArgs = [...args, "--url", url];
    console.log(`[wae-win32-x64] spawn ${cmd} ${fullArgs.join(" ")}`);
    return new Promise((resolve, reject) => {
        const child = spawn(cmd, fullArgs, {
            cwd,
            stdio: "inherit",
            env: {
                ...process.env,
                WAE_WINDOW_TITLE: title,
            },
            windowsHide: false,
        });
        child.on("error", reject);
        child.on("exit", (code: number | null) => {
            if (code === 0 || code === null) resolve();
            else reject(new Error(`wae-desktop exited with code ${code}`));
        });
    });
}

export const platform: WaePlatform = {
    id: "win32-x64",
    async start(options) {
        const url = options.url ?? "http://127.0.0.1:5173/";
        const runPromise = spawnDesktop(url, "WAE");
        return {
            async close() {
                await runPromise.catch(() => {});
            },
        };
    },
    async build(_options) {},
    async run(options) {
        const url = options.url ?? "http://127.0.0.1:5173/";
        const title = options.title ?? "WAE Desktop";
        await spawnDesktop(url, title);
    },
};

export default platform;

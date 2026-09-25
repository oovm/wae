/** CLI 入口：由 `bin/wae.mjs` 调用。 */

import { cmdRun } from "./cli/run.js";

const HELP = `WAE CLI

用法:
  wae create <name>
  wae dev [--platform <id>]
  wae build [--platform <id>]
  wae preview
  wae run [--platform <id>] [--port <n>] [--host <addr>] [--open|--no-open]
  wae check
  wae test
  wae generate [types]

当前已接线: run / dev（web 默认 Vite，可换；默认打开浏览器，可用 --no-open 关闭）
常用工具链: Vite（可换）· TypeScript · Rust/Cargo · wasm-bindgen
`;

export async function runCli(argv: string[]): Promise<void> {
    const [cmd = "help", ...args] = argv;
    const known = new Set([
        "create",
        "dev",
        "build",
        "preview",
        "run",
        "check",
        "test",
        "generate",
        "help",
        "--help",
        "-h",
    ]);

    if (!known.has(cmd)) {
        console.error(`未知命令: ${cmd}`);
        console.log(HELP);
        process.exitCode = 1;
        return;
    }

    if (cmd === "help" || cmd === "--help" || cmd === "-h") {
        console.log(HELP);
        return;
    }

    if (cmd === "run" || cmd === "dev") {
        await cmdRun(args, { mode: cmd === "dev" ? "dev" : "run" });
        return;
    }

    console.log(`wae ${cmd} ${args.join(" ")}（尚未接线）`.trim());
}

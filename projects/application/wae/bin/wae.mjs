#!/usr/bin/env node
/**
 * @wae/wae — 唯一项目 CLI。委托 Vite / Cargo / 平台包，不内置 TSX 编译器或 Rust runtime。
 */

const [cmd = "help", ...args] = process.argv.slice(2);

const help = `WAE CLI

用法:
  wae create <name>
  wae dev [--platform <id>]
  wae build [--platform <id>]
  wae preview
  wae run [--platform <id>]
  wae check
  wae test
  wae generate [types]

默认工具链: Vite · TypeScript · Rust/Cargo · wasm-bindgen
`;

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
    console.log(help);
    process.exitCode = 1;
} else if (cmd === "help" || cmd === "--help" || cmd === "-h") {
    console.log(help);
} else {
    console.log(`wae ${cmd} ${args.join(" ")}（骨架：后续接 Vite / 平台包）`.trim());
}

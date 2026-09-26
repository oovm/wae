/** WAE CLI command tree — Commander.js only; handlers injected by `@wae/wae`. */

import { Command } from "commander";

export type RunDevHandler = (args: string[], mode: "run" | "dev") => Promise<void>;
export type BuildHandler = (args: string[]) => Promise<void>;
export type StubHandler = (cmd: string, args: string[]) => void | Promise<void>;

export type WaeCommandHandlers = {
    run?: RunDevHandler;
    build?: BuildHandler;
    stub?: StubHandler;
};

function attachRunOptions(cmd: Command): Command {
    return cmd
        .option("-p, --platform <id>", "client platform id (web, win32-x64, …)")
        .option("--port <n>", "dev server port")
        .option("--host <addr>", "dev server bind address")
        .option("--open", "open browser (web default)")
        .option("--no-open", "do not open browser");
}

export function createWaeProgram(handlers: WaeCommandHandlers = {}): Command {
    const program = new Command()
        .name("wae")
        .description("WAE project CLI")
        .showHelpAfterError("(use wae help for usage)");

    const runAction = async (mode: "run" | "dev", _opts: unknown, cmd: Command) => {
        const args = rebuildRunArgv(cmd);
        if (handlers.run) {
            await handlers.run(args, mode);
            return;
        }
        (handlers.stub ?? defaultStub)(mode, args);
    };

    attachRunOptions(
        program
            .command("run")
            .description("Run app for current platform (web: Vite dev server)")
            .action(async (_opts, cmd) => runAction("run", _opts, cmd)),
    );

    attachRunOptions(
        program
            .command("dev")
            .description("Alias for wae run")
            .action(async (_opts, cmd) => runAction("dev", _opts, cmd)),
    );

    program
        .command("build")
        .description("Build shipped product (frontend + native addon + wae-product.json)")
        .option("-p, --platform <id>", "client platform id (web, win32-x64, …)")
        .option("--out-dir <dir>", "override product output base directory")
        .action(async (_opts, cmd) => {
            const args = rebuildBuildArgv(cmd);
            if (handlers.build) {
                await handlers.build(args);
                return;
            }
            (handlers.stub ?? defaultStub)("build", args);
        });

    for (const name of ["create", "preview", "check", "test", "generate"] as const) {
        program
            .command(name)
            .description(`${name} (skeleton)`)
            .allowUnknownOption(true)
            .action(async (_opts, cmd) => {
                (handlers.stub ?? defaultStub)(name, cmd.args.slice());
            });
    }

    return program;
}

function defaultStub(cmd: string, args: string[]): void {
    console.log(`wae ${cmd} ${args.join(" ")}（尚未接线）`.trim());
}

/** argv tail for `cmdRun` / `parseFlags` in `@wae/wae`. */
export function rebuildRunArgv(cmd: Command): string[] {
    const opts = cmd.opts() as {
        platform?: string;
        port?: string;
        host?: string;
        open?: boolean;
    };
    const out: string[] = [];
    if (opts.platform != null) out.push("--platform", opts.platform);
    if (opts.port != null) out.push("--port", String(opts.port));
    if (opts.host != null) out.push("--host", opts.host);
    if (opts.open === false) out.push("--no-open");
    else if (opts.open === true) out.push("--open");
    out.push(...cmd.args);
    return out;
}

/** argv tail for `cmdBuild`. */
export function rebuildBuildArgv(cmd: Command): string[] {
    const opts = cmd.opts() as { platform?: string; outDir?: string };
    const out: string[] = [];
    if (opts.platform != null) out.push("--platform", opts.platform);
    if (opts.outDir != null) out.push("--out-dir", opts.outDir);
    out.push(...cmd.args);
    return out;
}

/** Parse argv with the WAE command tree. */
export async function runWaeCli(argv: string[], handlers: WaeCommandHandlers): Promise<void> {
    const program = createWaeProgram(handlers);
    await program.parseAsync(["node", "wae", ...argv]);
}

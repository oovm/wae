/** 应用生命周期协议（框架无关）。 */

export type AppLifecycleHooks = {
    onReady?: () => void | Promise<void>;
    onSuspend?: () => void | Promise<void>;
    onResume?: () => void | Promise<void>;
    onShutdown?: () => void | Promise<void>;
};

export type AppLifecycle = {
    ready(): Promise<void>;
    suspend(): Promise<void>;
    resume(): Promise<void>;
    shutdown(): Promise<void>;
};

export function createLifecycle(hooks: AppLifecycleHooks = {}): AppLifecycle {
    return {
        async ready() {
            await hooks.onReady?.();
        },
        async suspend() {
            await hooks.onSuspend?.();
        },
        async resume() {
            await hooks.onResume?.();
        },
        async shutdown() {
            await hooks.onShutdown?.();
        },
    };
}

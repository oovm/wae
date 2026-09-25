/** 请求级生命周期：cold start 钩子、abort、waitUntil 登记。 */

export type LifecycleHooks = {
    onColdStart?: () => void | Promise<void>;
};

let cold = true;

export async function runWithLifecycle<T>(hooks: LifecycleHooks, run: () => Promise<T>): Promise<T> {
    if (cold) {
        cold = false;
        await hooks.onColdStart?.();
    }
    return run();
}

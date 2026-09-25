/** 环境绑定抽象（KV/R2/D1 等由具体 runtime adapter 注入，不在此写死厂商类型）。 */

export type BindingMap = Record<string, unknown>;

export function readBinding<T>(env: BindingMap, name: string): T | undefined {
    return env[name] as T | undefined;
}

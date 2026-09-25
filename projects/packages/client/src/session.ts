/** Session 接口（框架无关；实现由 server / adapter 注入）。 */

export type SessionClient = {
    get<T = unknown>(key: string): Promise<T | undefined>;
    set(key: string, value: unknown): Promise<void>;
    clear(): Promise<void>;
};

export function createMemorySession(): SessionClient {
    const map = new Map<string, unknown>();
    return {
        async get(key) {
            return map.get(key) as never;
        },
        async set(key, value) {
            map.set(key, value);
        },
        async clear() {
            map.clear();
        },
    };
}

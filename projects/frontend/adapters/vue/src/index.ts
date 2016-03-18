import type { WaeClient } from "@wae/client";

const KEY = Symbol("wae");

export function provideWae(app: { provide(k: symbol, v: unknown): void }, client: WaeClient) {
    app.provide(KEY, client);
}

export function useWae(): WaeClient {
    throw new Error("useWae: wire to Vue inject in real implementation");
}

export default function vue(): { name: "vue" } {
    return { name: "vue" };
}

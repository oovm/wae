import type { WaeClient } from "@wae/client";

export type WaeProviderProps = {
    client: WaeClient;
    children?: unknown;
};

export function WaeProvider(_props: WaeProviderProps): null {
    return null;
}

export function useWae(): WaeClient {
    throw new Error("useWae: wire to React context in real implementation");
}

/** 供 defineConfig({ frontend: { adapter: react() } }) 使用 */
export default function react(): { name: "react" } {
    return { name: "react" };
}

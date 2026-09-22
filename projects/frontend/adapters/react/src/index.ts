import {
    createContext,
    createElement,
    useContext,
    type ReactNode,
} from "react";
import type { WaeClient } from "@wae/client";

const WaeContext = createContext<WaeClient | null>(null);

export type WaeProviderProps = {
    client: WaeClient;
    children?: ReactNode;
};

export function WaeProvider(props: WaeProviderProps) {
    return createElement(WaeContext.Provider, { value: props.client }, props.children);
}

export function useWae(): WaeClient {
    const client = useContext(WaeContext);
    if (!client) {
        throw new Error("useWae() 需要包裹在 <WaeProvider client={…}> 内");
    }
    return client;
}

/** 供 defineConfig({ frontend: { adapter: react() } }) 使用 */
export default function react(): { name: "react" } {
    return { name: "react" };
}

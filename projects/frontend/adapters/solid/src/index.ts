import type { WaeClient } from "@wae/client";

export function WaeProvider(_props: { client: WaeClient; children?: unknown }): null {
    return null;
}

export function useWae(): WaeClient {
    throw new Error("useWae: solid context skeleton");
}

export default function solid(): { name: "solid" } {
    return { name: "solid" };
}

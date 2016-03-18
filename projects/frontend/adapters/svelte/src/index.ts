import type { WaeClient } from "@wae/client";

export function setWaeContext(_client: WaeClient): void {
    // Svelte context.set 由实现补齐
}

export function getWaeContext(): WaeClient {
    throw new Error("getWaeContext: skeleton");
}

export default function svelte(): { name: "svelte" } {
    return { name: "svelte" };
}

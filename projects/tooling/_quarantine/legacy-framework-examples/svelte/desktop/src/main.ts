import { createRuntime } from "@wae/client";
import { setWaeContext } from "@wae/adapter-svelte";

/** 验证：WAE runtime 框架无关 + svelte adapter 可接入（desktop）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "desktop", hasNativeBridge: true },
});

void wae;
void setWaeContext;

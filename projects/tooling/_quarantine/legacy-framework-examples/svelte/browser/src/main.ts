import { createRuntime } from "@wae/client";
import { setWaeContext } from "@wae/adapter-svelte";

/** 验证：WAE runtime 框架无关 + svelte adapter 可接入（browser）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "web", hasNativeBridge: false },
});

void wae;
void setWaeContext;

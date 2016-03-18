import { createRuntime } from "@wae/client";
import { attachWae } from "@wae/adapter-vanilla";

/** 验证：WAE runtime 框架无关 + vanilla adapter 可接入（browser）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "web", hasNativeBridge: false },
});

void wae;
void attachWae;

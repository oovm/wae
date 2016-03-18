import { createRuntime } from "@wae/client";
import { attachWae } from "@wae/adapter-vanilla";

/** 验证：WAE runtime 框架无关 + vanilla adapter 可接入（desktop）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "desktop", hasNativeBridge: true },
});

void wae;
void attachWae;

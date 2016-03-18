import { createRuntime } from "@wae/client";
import { provideWae } from "@wae/adapter-vue";

/** 验证：WAE runtime 框架无关 + vue adapter 可接入（fullstack）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "web", hasNativeBridge: false },
});

void wae;
void provideWae;

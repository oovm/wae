import { createRuntime } from "@wae/client";
import { WaeProvider } from "@wae/adapter-react";

/** 验证：WAE runtime 框架无关 + react adapter 可接入（desktop）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "desktop", hasNativeBridge: true },
});

void wae;
void WaeProvider;

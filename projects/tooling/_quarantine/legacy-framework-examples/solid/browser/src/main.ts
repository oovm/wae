import { createRuntime } from "@wae/client";
import { WaeProvider } from "@wae/adapter-solid";

/** 验证：WAE runtime 框架无关 + solid adapter 可接入（browser）。 */
const wae = createRuntime({
  server: { baseUrl: "/api" },
  env: { target: "web", hasNativeBridge: false },
});

void wae;
void WaeProvider;

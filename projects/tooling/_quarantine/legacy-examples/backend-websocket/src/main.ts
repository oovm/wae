import { createServer, route } from "@wae/server";

/** WebSocket 契约验证骨架（正式协议接入后替换）。 */
export const app = createServer({
  routes: [
    route("GET", "/ws/health", (ctx) => ctx.json({ protocol: "websocket-skeleton" })),
  ],
});

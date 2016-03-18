import { createServer, route } from "@wae/server";

export const app = createServer({
  routes: [
    route("POST", "/rpc/ping", (ctx) => ctx.json({ pong: true })),
  ],
});

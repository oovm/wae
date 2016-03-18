import { createServer, route } from "@wae/server";
import { serve } from "@wae/server-node";

export const app = createServer({
  routes: [
    route("GET", "/api/health", (ctx) => ctx.json({ ok: true })),
  ],
});

export function start() {
  return serve(app, { port: 3000 });
}

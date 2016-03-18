import { createServer, route } from "@wae/server";
import { createWorker } from "@wae/server-cloudflare";

const app = createServer({
  routes: [
    route("GET", "/api/health", (ctx) => ctx.json({ ok: true })),
  ],
});

export default createWorker(app);

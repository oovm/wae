import { createServer, route } from "@wae/server";

export const app = createServer({
  routes: [
    route("GET", "/", (ctx) =>
      ctx.text("<!doctype html><html><body><div id=\"root\"></div></body></html>"),
    ),
  ],
});

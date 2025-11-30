import { Hono } from "hono";
import { compress } from "hono/compress";

import { serveStatic } from "./serve-static";

const app = new Hono();

app.use(compress());

app.get("/assets/*", async (c) => {
  return serveStatic(c);
});

app.get("/favicon.ico", async (c) => {
  // return 404
  return c.notFound();
});

export { app };

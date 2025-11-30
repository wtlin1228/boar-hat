import * as fs from "node:fs";
import path from "path";

import { serve } from "@hono/node-server";

import { app } from "./app";
import { simpleSSR } from "./simple-ssr";
import { simpleSSRWithHydration } from "./simple-ssr-hydration";

const dist = path.join(process.cwd(), "./dist/client");
app.get("/*", async (c) => {
  const html = fs.readFileSync(`${dist}/index.html`).toString();

  /**
   * Uncomment to use simple SSR
   */
  // return c.html(simpleSSR(c, html));

  /**
   * Uncomment to use simple SSR with data hydration
   */
  // return c.html(simpleSSRWithHydration(c, html));

  return c.html(html);
});

const createServer = async () => {
  const port = process.env.PORT ? Number(process.env.PORT) : 3000;
  console.log(`Server is running on http://localhost:${port}`);

  serve({
    fetch: app.fetch,
    port,
  });
};

createServer().catch((error) => {
  console.error("Error starting server:", error);
  process.exit(1);
});

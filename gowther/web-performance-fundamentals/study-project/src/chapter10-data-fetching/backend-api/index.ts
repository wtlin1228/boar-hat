import { sidebarData } from "@fe/data/sidebar";
import { tableData } from "@fe/data/website-statistics";
import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { compress } from "hono/compress";

const app = new Hono();

app.use(compress());

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

app.get("/api/sidebar", async (c) => {
  await sleep(500);

  // enable cors for all
  c.res.headers.set("Access-Control-Allow-Origin", "*");

  return c.json(sidebarData);
});

app.get("/api/statistics", async (c) => {
  await sleep(700);

  c.res.headers.set("Access-Control-Allow-Origin", "*");

  return c.json(tableData);
});

const createServer = async () => {
  const port = process.env.PORT ? Number(process.env.PORT) : 5432;
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

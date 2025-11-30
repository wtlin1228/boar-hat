import fs from "node:fs";
import path from "path";

import { Context } from "hono";

const dist = path.join(process.cwd(), "./dist/client");
export const serveStatic = async (c: Context) => {
  const resourcePath = path.join(`${dist}`, c.req.path);

  const resourceContent = fs.readFileSync(resourcePath).toString();

  // Set cache headers
  c.header("Cache-Control", "max-age=9000000");

  // Set MIME type
  if (resourcePath.endsWith(".css")) {
    c.header("Content-Type", "text/css");
  } else if (resourcePath.endsWith(".js")) {
    c.header("Content-Type", "application/javascript");
  }

  return c.body(resourceContent, 200);
};

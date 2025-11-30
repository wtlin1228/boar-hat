import fs from "fs";
import path from "path";

import React from "react";

import { Context } from "hono";
import { renderToString } from "react-dom/server";

const dist = path.join(process.cwd(), "./dist/server");

export const simpleSSRWithHydration = async (c: Context, html: string) => {
  // bust the cache to imitate a cold start
  // otherwise, node cashes the import until the server is restarted
  const { default: App } = await import(`${dist}/App?cache=${Math.random()}`);

  const sidebarPromise = fetch(`http://localhost:5432/api/sidebar`).then(
    (res) => res.json(),
  );
  const statisticsPromise = fetch(`http://localhost:5432/api/statistics`).then(
    (res) => res.json(),
  );

  const [sidebar, statistics] = await Promise.all([
    sidebarPromise,
    statisticsPromise,
  ]);

  const reactHtml = renderToString(
    React.createElement(App, { ssrPath: c.req.path, sidebar, statistics }),
  );

  const htmlWithData = `
  <script>window.__SSR_DATA__ = ${JSON.stringify({
    sidebar,
    statistics,
  })}</script>
  ${reactHtml}`;

  const finalHtml = html.replace("<!--ssr-->", htmlWithData);

  fs.writeFileSync(`${dist}/ssr.html`, finalHtml);
  return finalHtml;
};

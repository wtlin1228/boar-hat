import p from "path";

import React from "react";

import { renderToString } from "react-dom/server";
import { createServer as createViteServer, ViteDevServer } from "vite";

export const preRenderApp = async (html: string, path: string) => {
  const { default: App } = await import(
    `${p.join(process.cwd(), "./dist/server")}/App.js`
  );

  const reactHtml = renderToString(React.createElement(App, { ssrPath: path }));

  const finalHtml = html.replace(
    '<div id="root"></div>',
    `<div id="root">${reactHtml}</div>`,
  );

  return finalHtml;
};

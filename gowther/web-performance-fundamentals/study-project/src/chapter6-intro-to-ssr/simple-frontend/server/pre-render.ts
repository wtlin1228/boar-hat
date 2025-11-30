import p from "path";

import React from "react";

import { renderToString } from "react-dom/server";
import { createServer as createViteServer, ViteDevServer } from "vite";

let vite: ViteDevServer;

export const preRenderApp = async (html: string, path: string) => {
  vite =
    vite ??
    (await createViteServer({
      server: { middlewareMode: true },
      appType: "custom",
    }));

  const { default: App } = await vite.ssrLoadModule(
    p.join(process.cwd(), "App.tsx"),
  );

  const reactHtml = renderToString(React.createElement(App, { ssrPath: path }));

  const finalHtml = html.replace(
    '<div id="root"></div>',
    `<div id="root">${reactHtml}</div>`,
  );

  return finalHtml;
};

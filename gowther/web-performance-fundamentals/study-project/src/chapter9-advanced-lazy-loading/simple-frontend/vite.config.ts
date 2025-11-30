import path, { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import react from "@vitejs/plugin-react";
import { visualizer } from "rollup-plugin-visualizer";
import { defineConfig } from "vite";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const FRONTEND_ROOT = path.resolve(__dirname, "../../../frontend");

// https://vite.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@fe": FRONTEND_ROOT,
    },
  },
  ssr: {
    noExternal: [
      "@mui/system",
      "@mui/utils",
      "@mui/material",
      "@mui/icons-material",
      "@mui/styled-engine",
    ],
  },
  server: {
    port: 1234,
  },
  define: {
    process: {
      env: {
        NODE_ENV: JSON.stringify("development"),
        VITE_APP_VERSION: JSON.stringify("1.0.0"),
      },
    },
  },
  preview: {
    headers: {
      "Cache-Control": "max-age=9000000",
    },
  },
  plugins: [
    react(),
    visualizer({
      filename: "stats.html",
      emitFile: true,
      template: "treemap",
    }),
  ],
  build: {
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          if (id.includes("@radix")) {
            return "radix";
          }
          if (id.includes("@tiptap") || id.includes("prosemirror")) {
            return "editor";
          }
          if (id.includes("date-fns")) {
            return "date-fns";
          }
          if (id.includes("node_modules")) {
            return "vendor";
          }

          return null;
        },
      },
    },
  },
});

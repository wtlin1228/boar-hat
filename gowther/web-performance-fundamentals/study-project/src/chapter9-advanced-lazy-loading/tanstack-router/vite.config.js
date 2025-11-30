import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import path from "path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const FRONTEND_ROOT = path.resolve(__dirname, "../../../frontend");

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@fe": FRONTEND_ROOT,
    },
  },
  preview: {
    headers: {
      "Cache-Control": "max-age=9000000",
    },
  },
  plugins: [
    TanStackRouterVite({ target: "react", autoCodeSplitting: true }),
    react(),
  ],
});

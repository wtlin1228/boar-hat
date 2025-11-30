import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import path from "path";

import type { NextConfig } from "next";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const FRONTEND_ROOT = path.resolve(__dirname, "../../../../frontend");

const nextConfig: NextConfig = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true, // Disables TypeScript type checking during builds
  },
  transpilePackages: ["@fe"],
};

export default nextConfig;

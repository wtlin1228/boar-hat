import { defineConfig } from "@rsbuild/core";
import { pluginReact } from "@rsbuild/plugin-react";
import { pluginModuleFederation } from "@module-federation/rsbuild-plugin";
import moduleFederationConfig from "./module-federation.config";

export default defineConfig({
  output: {
    minify: false,
    assetPrefix: "http://localhost:3002/",
  },
  plugins: [pluginReact(), pluginModuleFederation(moduleFederationConfig, {})],
  server: {
    port: 3002,
  },
});

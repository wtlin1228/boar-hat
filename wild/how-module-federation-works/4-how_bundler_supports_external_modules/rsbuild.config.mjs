import { defineConfig } from "@rsbuild/core";

export default defineConfig({
  html: { template: "./public/index.html" },
  output: {
    minify: false,
    externals: {
      lodash: "_",
    },
  },
});

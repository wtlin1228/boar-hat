import { defineConfig } from "@rsbuild/core";

export default defineConfig({
  source: {
    main: "./src/index.js",
  },
  output: {
    minify: false,
  },
  tools: {
    rspack: {
      optimization: {
        splitChunks: {
          cacheGroups: {
            m2: {
              test: /[\\/]m2\.js$/,
              chunks: "all",
              enforce: true,
            },
          },
        },
      },
    },
  },
});

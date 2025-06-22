import { defineConfig } from '@rsbuild/core';

export default defineConfig({
  source: {
    main: './src/index.js',
  },
  output: {
    minify: false,
  },
  tools: {
    rspack: {
      optimization: {
        splitChunks: {
          cacheGroups: {
            m1: {
              test: /[\\/]m1\.js$/,
              name: 'm1',
              chunks: 'all',
              enforce: true,
            },
          },
        },
      },
    },
  },
});

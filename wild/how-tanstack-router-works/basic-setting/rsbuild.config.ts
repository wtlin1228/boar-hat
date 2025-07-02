import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { tanstackRouter } from '@tanstack/router-plugin/rspack';

export default defineConfig({
  plugins: [pluginReact()],
  tools: {
    rspack: {
      plugins: [
        tanstackRouter({
          target: 'react',
          routesDirectory: './src/routes',
          generatedRouteTree: './src/routeTree.gen.ts',
          routeFileIgnorePrefix: '-',
          quoteStyle: 'single',
          // autoCodeSplitting: true,
        }),
      ],
    },
  },
});

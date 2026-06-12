import { resolve } from 'node:path'

import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'
import dts from 'vite-plugin-dts'

// The library build is gated behind `--mode lib` (see the `build` npm script).
// Storybook also loads this file, so keeping the `lib`/`external` settings out
// of its build (mode is `production`/`development` there) avoids breaking the
// Storybook preview bundle.
// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const isLibBuild = mode === 'lib'

  if (!isLibBuild) {
    return { plugins: [react()] }
  }

  return {
    plugins: [
      react(),
      dts({
        tsconfigPath: './tsconfig.build.json',
        entryRoot: resolve(__dirname, 'src'),
        insertTypesEntry: true,
      }),
    ],
    build: {
      lib: {
        entry: resolve(__dirname, 'src/index.ts'),
        formats: ['es'],
        fileName: 'index',
        cssFileName: 'styles',
      },
      rollupOptions: {
        external: ['react', 'react-dom', 'react/jsx-runtime'],
      },
    },
  }
})

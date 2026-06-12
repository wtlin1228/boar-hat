import { fileURLToPath } from 'node:url'

import react from '@vitejs/plugin-react'
import { storybookTest } from '@storybook/addon-vitest/vitest-plugin'
import { playwright } from '@vitest/browser-playwright'
import { defineConfig } from 'vitest/config'

const storybookConfigDir = fileURLToPath(new URL('.storybook', import.meta.url))

// A fresh browser config per project — Vitest derives project names from the
// instances, so the objects must not be shared between projects.
const browser = () => ({
  enabled: true,
  headless: true,
  provider: playwright(),
  instances: [{ browser: 'chromium' as const }],
})

// Two browser-mode projects:
// - `unit`      runs the *.test.tsx files next to each component.
// - `storybook` runs every story as a test via the Storybook Vitest addon
//   (project annotations from `.storybook/preview` are applied automatically).
// More info: https://storybook.js.org/docs/writing-tests/integrations/vitest-addon
export default defineConfig({
  test: {
    projects: [
      {
        plugins: [react()],
        test: {
          name: 'unit',
          include: ['src/**/*.{test,spec}.{ts,tsx}'],
          browser: browser(),
        },
      },
      {
        plugins: [storybookTest({ configDir: storybookConfigDir })],
        test: {
          name: 'storybook',
          browser: browser(),
        },
      },
    ],
  },
})

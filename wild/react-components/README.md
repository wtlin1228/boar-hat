# react-components

A React component library built with **Vite** (library mode), **Storybook 10**,
and **Vitest 4** browser-mode testing (Playwright). TypeScript, ESLint and
Prettier are configured out of the box.

## Requirements

- Node.js `>=20.19` (or `>=22.12`)
- [pnpm](https://pnpm.io) `>=10`
- A Chromium binary for browser tests: `pnpm exec playwright install chromium`

## Getting started

```bash
pnpm install
pnpm storybook   # develop components in Storybook at http://localhost:6006
```

## Project structure

```
src/
  index.ts                     # public entry point (barrel of all exports)
  components/
    DropZone/
      DropZone.tsx             # the component
      DropZone.css             # component styles
      DropZone.stories.tsx     # Storybook stories
      DropZone.test.tsx        # Vitest browser tests
      index.ts                 # component barrel
```

To add a component, create a folder under `src/components/`, then re-export it
from `src/index.ts`.

## Scripts

| Script                 | Description                                       |
| ---------------------- | ------------------------------------------------- |
| `pnpm storybook`       | Start Storybook in dev mode                       |
| `pnpm build-storybook` | Build the static Storybook site                   |
| `pnpm build`           | Type-check and build the library to `dist/`       |
| `pnpm test`            | Run unit + story tests once (Playwright/Chromium) |
| `pnpm test:watch`      | Run tests in watch mode                           |
| `pnpm test:coverage`   | Run tests with coverage                           |
| `pnpm typecheck`       | Type-check without emitting                       |
| `pnpm lint`            | Run ESLint                                        |
| `pnpm format`          | Format all files with Prettier                    |

## Testing

Tests run in a real browser via Vitest's browser mode (Playwright + Chromium).
Two projects are defined in [`vitest.config.ts`](./vitest.config.ts):

- **unit** — the `*.test.tsx` files next to each component.
- **storybook** — every story is run as a test by the
  [`@storybook/addon-vitest`](https://storybook.js.org/docs/writing-tests/integrations/vitest-addon)
  plugin.

## Building & consuming

`pnpm build` outputs ES modules and type declarations to `dist/`. Styles are
emitted to `dist/styles.css`. `react` / `react-dom` are external (declared as
peer dependencies).

```tsx
import { DropZone } from 'react-components'
import 'react-components/styles.css'

export function App() {
  return <DropZone label="Drop a file" onChange={console.log} />
}
```

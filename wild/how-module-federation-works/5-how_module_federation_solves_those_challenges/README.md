# Setup

To run the demo locally:

1. Start **Kirby**:

   ```sh
   cd kirby && pnpm i && pnpm build && pnpm preview
   ```

2. Start **Pikmin**:

   ```sh
   cd pikmin && pnpm i && pnpm build && pnpm preview
   ```

3. Start **Host**:

   ```sh
   cd host && pnpm i && pnpm build && pnpm preview
   ```

4. Open your browser and navigate to [http://localhost:3000](http://localhost:3000)

---

# Explanation

This project demonstrates how Module Federation handles shared dependencies—in this case, different versions of `lodash`.

### Case 1: Load Order - Kirby → Pikmin

- On first load, no `lodash` is downloaded.
- Click the `Toggle Kirby` button → `lodash@4.0.0` is fetched from `http://localhost:3001` and registered.
- Then click the `Toggle Pikmin` button → `lodash@4.17.21` is fetched from `http://localhost:3002` and registered.
- Check the shared modules via the browser console:

  ```js
  __FEDERATION__.__SHARE__;
  ```

  You'll see something like:

  ```json
  {
    "4.0.0": {
      "useIn": ["kirby"],
      "from": "kirby",
      "version": "4.0.0",
      "loaded": true,
      "strategy": "version-first",
      ...
    },
    "4.17.21": {
      "useIn": ["pikmin"],
      "from": "pikmin",
      "version": "4.17.21",
      "loaded": true,
      "strategy": "version-first",
      ...
    }
  }
  ```

### Case 2: Load Order - Pikmin → Kirby

- Click the `Toggle Pikmin` button first → `lodash@4.17.21` is loaded.
- Then click the `Toggle Kirby` button → **Kirby reuses Pikmin's lodash**, so `lodash@4.0.0` is **not** downloaded.
- Check the shared registry again with `__FEDERATION__.__SHARE__`, and you'll see:

  ```json
  {
    "4.17.21": {
      "useIn": ["pikmin", "kirby"],
      "from": "pikmin",
      "version": "4.17.21",
      "loaded": true,
      ...
    },
    "4.0.0": {
      "useIn": [],
      "from": "kirby",
      "version": "4.0.0",
      "strategy": "version-first",
      ...
    }
  }
  ```

> Since `lodash@4.17.21` satisfies the required version for Kirby (`^4.0.0`), Kirby skips loading its own copy and reuses Pikmin’s version.

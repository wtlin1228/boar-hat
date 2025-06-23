# How Module Federation Works

## How Bundler Loads Modules

Before diving into how Module Federation works, it's important to first understand how a bundler loads modules.

### Entry Code

Here’s a simple example of an entry module:

```js
import "./index.css";
import { a, b } from "./m1";

document.querySelector("#root").innerHTML = `
  <div class="content">
    <h1>Vanilla Rsbuild</h1>
    <p>Start building amazing things with Rsbuild.</p>
    <p>${a}</p>
    <p>${b}</p>
  </div>
`;
```

After bundling, this entry module gets transformed into something like:

```js
var __webpack_modules__ = {
  681: function (
    __unused_webpack_module,
    __unused_webpack___webpack_exports__,
    __webpack_require__
  ) {
    // CSS is extracted by css-extract-rspack-plugin

    // Load m1.js as an external module
    var m1 = __webpack_require__(890);

    // Entry module logic
    document.querySelector("#root").innerHTML = `
      <div class="content">
        <h1>Vanilla Rsbuild</h1>
        <p>Start building amazing things with Rsbuild.</p>
        <p>${m1.a}</p>
        <p>${m1.b}</p>
      </div>
    `;
  },
};
```

### m1.js Source and Bundled Output

Source:

```js
// src/m1.js
export const a = "a";
export const b = a + "b";
```

Bundled:

```js
"use strict";
(self["webpackChunksingle_module"] =
  self["webpackChunksingle_module"] || []).push([
  ["354"],
  {
    890: function (
      __unused_webpack_module,
      __webpack_exports__,
      __webpack_require__
    ) {
      __webpack_require__.d(__webpack_exports__, {
        a: () => a,
        b: () => b,
      });
      const a = "a";
      const b = a + "b";
    },
  },
]);
```

When this script is loaded in the browser, it pushes a new chunk into the global array `webpackChunksingle_module`. Specifically, it pushes:

```js
[["354"], { 890: function (...) { ... } }]
```

This is how the bundler’s runtime system knows which chunk ID (`"354"`) corresponds to which module definitions (`{ 890: ... }`).

Once pushed, the runtime will register module `890` and mark chunk `"354"` as loaded. Then, any code that depends on `"354"` can safely execute.

### Script Loading Order

Critical and chunked JavaScript resources are loaded using `defer`, ensuring they're downloaded in parallel but executed in the correct order:

```html
<script defer src="/static/js/m1.22366322.js"></script>
<script defer src="/static/js/index.f7f8c54b.js"></script>
```

### Runtime

- `define_property_getters`: set the getter for each property of the definition
- `has_own_property`: just a wrapper over `Object.hasOwnProperty`
- `on_chunk_loaded`: manages when to execute code that depends on one or more asynchronously loaded chunks
- `rspack_version`: get rspack's version, ex: `'1.3.12'`
- `jsonp_chunk_loading`: handles loading additional JavaScript chunks asynchronously, essential for features like code splitting and Module Federation
- `rspack_unique_id`: get rspack's unique id, ex: `'bundler=rspack@1.3.12'`

#### `define_property_getters`

```js
__webpack_require__.d = (exports, definition) => {
  for (var key in definition) {
    if (
      __webpack_require__.o(definition, key) &&
      !__webpack_require__.o(exports, key)
    ) {
      Object.defineProperty(exports, key, {
        enumerable: true,
        get: definition[key],
      });
    }
  }
};
```

#### `has_own_property`

```js
__webpack_require__.o = (obj, prop) =>
  Object.prototype.hasOwnProperty.call(obj, prop);
```

#### `on_chunk_loaded`

This mechanism allows code to register a callback (`fn`) to be executed only when all of the specified chunks (`chunkIds`) are fully loaded. It also supports prioritizing tasks with the optional `priority` value.

```js
/**
 * This is a queue of delayed callbacks that look like this: [chunkIds, fn, priority]
 * Where:
 * - chunkIds: Array of chunk IDs this callback depends on
 * - fn: Function to call once the chunks are loaded
 * - priority: Used to control execution order
 */
var deferred = [];

/**
 * This is both a callback registration function and a scheduler. It behaves differently
 * depending on whether chunkIds is passed:
 * 1. Register a callback (when chunkIds is given)
 * 2. Check and run fulfilled callbacks (when chunkIds is not passed)
 */
__webpack_require__.O = (result, chunkIds, fn, priority) => {
  if (chunkIds) {
    // ex: __webpack_require__.O(undefined, ["354"], () => __webpack_require__(681), 0);
    // Insert the new callback into the `deferred` queue in order of `priority`.
    // higher priority -> placed earlier in the list.
    // lower priority -> pushed toward the end.
    priority = priority || 0;
    for (var i = deferred.length; i > 0 && deferred[i - 1][2] > priority; i--)
      deferred[i] = deferred[i - 1];
    deferred[i] = [chunkIds, fn, priority];

    // This call does not run the function yet, it simply defers it until the required
    // chunks are loaded.
    return;
  }

  // ex: var result = __webpack_require__.O();
  var notFulfilled = Infinity;
  for (var i = 0; i < deferred.length; i++) {
    var [chunkIds, fn, priority] = deferred[i];
    var fulfilled = true;
    for (var j = 0; j < chunkIds.length; j++) {
      if (
        (priority & (1 === 0) || notFulfilled >= priority) &&
        // - This calls all check functions in __webpack_require__.O
        //   (like __webpack_require__.O.j) on each chunkId.
        // - If all return true, the chunk is considered "fulfilled".
        Object.keys(__webpack_require__.O).every((key) =>
          __webpack_require__.O[key](chunkIds[j])
        )
      ) {
        chunkIds.splice(j--, 1);
      } else {
        fulfilled = false;
        if (priority < notFulfilled) notFulfilled = priority;
      }
    }
    if (fulfilled) {
      deferred.splice(i--, 1);
      var r = fn();
      if (r !== undefined) result = r;
    }
  }
  return result;
};
```

#### `rspack_version`

```js
__webpack_require__.rv = () => "1.3.12";
```

#### `jsonp_chunk_loading`

```js
// object to store loaded and loading chunks
// - undefined = chunk not loaded
// - null = chunk preloaded/prefetched
// - [resolve, reject, Promise] = chunk loading
// - 0 = chunk loaded
var installedChunks = { 980: 0 };

/**
 * This function checks if a chunk is already loaded. It's used to
 * determine if it's safe to run modules that depend on this chunk.
 *
 * There could be other functions for checking whether a chunk has
 * been loaded with more complex setting.
 */
__webpack_require__.O.j = (chunkId) => installedChunks[chunkId] === 0;

/**
 * This function is called when a new chunk is loaded. The data array contains:
 * - chunkIds: An array of chunk IDs included in this file
 * - moreModules: An object mapping module IDs to factory functions
 * - runtime: (Optional) a function to run after modules are registered
 */
var webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
  var [chunkIds, moreModules, runtime] = data;
  // add "moreModules" to the modules object,
  // then flag all "chunkIds" as loaded and fire callback
  var moduleId,
    chunkId,
    i = 0;
  if (chunkIds.some((id) => installedChunks[id] !== 0)) {
    for (moduleId in moreModules) {
      if (__webpack_require__.o(moreModules, moduleId)) {
        // register module
        __webpack_require__.m[moduleId] = moreModules[moduleId];
      }
    }
    if (runtime) var result = runtime(__webpack_require__);
  }
  if (parentChunkLoadingFunction) parentChunkLoadingFunction(data);
  for (; i < chunkIds.length; i++) {
    chunkId = chunkIds[i];
    if (
      __webpack_require__.o(installedChunks, chunkId) &&
      installedChunks[chunkId]
    ) {
      // resolve the promise
      installedChunks[chunkId][0]();
    }
    // mark as loaded
    installedChunks[chunkId] = 0;
  }

  // This resumes any pending module executions waiting for this chunk.
  return __webpack_require__.O(result);
};

// This is the global array used to buffer pushed chunks, usually named
// like webpackChunk<name>.
var chunkLoadingGlobal = (self["webpackChunksingle_module"] =
  self["webpackChunksingle_module"] || []);

// process any chunks already in the array
chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));

// override .push() to use our callback
chunkLoadingGlobal.push = webpackJsonpCallback.bind(
  null,
  chunkLoadingGlobal.push.bind(chunkLoadingGlobal)
);
```

#### `rspack_unique_id`

```js
__webpack_require__.ruid = "bundler=rspack@1.3.12";
```

### Startup Code

To bootstrap the application, the bundler delays execution of the entry module until all required chunks (like `m1.js`) are available:

```js
// Startup code
// Load the entry module after dependencies are ready
var __webpack_exports__ = __webpack_require__.O(
  undefined,
  ["354"], // dependencies
  function () {
    return __webpack_require__(681); // entry module
  }
);
__webpack_exports__ = __webpack_require__.O(__webpack_exports__);
```

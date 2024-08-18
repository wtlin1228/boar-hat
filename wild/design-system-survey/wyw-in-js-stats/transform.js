// transform do `prepare code` -> `resolve imports` -> `process imports`

// transform call the prepare function first
// then get the [preparedCode, imports, metadata]

// resolve the imports:
const imports = new Map([["./get-color", ["getColor"]]]);
// resolved to:
const resolvedImports = [
  {
    source: "./get-color",
    only: ["getColor"],
    resolved:
      "/Users/linweitang/GitHub/wyw-in-js/packages/transform/src/leo-is-here/__fixtures__/get-color.js",
  },
];

// After all imports are resolved, can do process imports now.
// For each resolved import, create a new entrypoint for it as the child of current entrypoint.
// Don't process the child entrypoint if there is a cycle.
//
// In this case, we have only one import to process, and only need to process the "getColor".

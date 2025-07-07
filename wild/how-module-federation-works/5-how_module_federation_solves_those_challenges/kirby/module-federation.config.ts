import { createModuleFederationConfig } from "@module-federation/rsbuild-plugin";
import { dependencies } from "./package.json";

export default createModuleFederationConfig({
  name: "kirby",
  exposes: {
    ".": "./src/components/exposed.tsx",
  },
  shared: {
    react: { singleton: true },
    "react-dom": { singleton: true },
    lodash: dependencies.lodash,
  },
  getPublicPath: `function() { return "http://localhost:3001/"; }`,
});

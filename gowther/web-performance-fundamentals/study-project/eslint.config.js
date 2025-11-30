import js from "@eslint/js";
import importRules from "eslint-plugin-import";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import react from "eslint-plugin-react";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config({
  extends: [
    js.configs.recommended,
    ...tseslint.configs.recommended,
    react.configs.flat.recommended,
  ],
  files: ["**/*.{ts,tsx}"],
  ignores: [
    "**/dist/**",
    "**/.next/**",
    "**/routeTree.gen.ts",
    "**/node_modules/**",
  ],
  languageOptions: {
    ecmaVersion: 2020,
    globals: globals.browser,
  },
  plugins: {
    "react-hooks": reactHooks,
    "react-refresh": reactRefresh,
    import: importRules,
    react: react,
  },
  settings: {
    react: {
      version: "18",
    },
  },
  rules: {
    ...reactHooks.configs.recommended.rules,
    "react-refresh/only-export-components": [
      "warn",
      { allowConstantExport: true },
    ],
    ...importRules.configs.errors.rules,
    ...importRules.configs.typescript.rules,
    "import/namespace": "off",
    "import/newline-after-import": "error",
    "import/no-duplicates": "error",
    "import/no-unresolved": "off",
    "import/default": "off",
    "react/prop-types": "off",
    "react/no-unused-prop-types": "off",
    "react/react-in-jsx-scope": "off",
    "react/no-unescaped-entities": "off",
    "react-refresh/only-export-components": "off",
    "react/display-name": "off",
    "import/order": [
      "error",
      {
        "newlines-between": "always",
        pathGroups: [
          {
            pattern: "{react,react-*,aws-*}",
            group: "external",
            position: "before",
          },
        ],
        pathGroupsExcludedImportTypes: ["builtin"],
        alphabetize: { order: "asc", caseInsensitive: true },
      },
    ],
    "import/no-useless-path-segments": "error",
    "@typescript-eslint/no-unused-vars": "off",
  },
});

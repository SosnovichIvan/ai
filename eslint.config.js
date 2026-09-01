import js from "@eslint/js";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";

export default [
  { ignores: ["dist", "src-tauri/target", "src-tauri/icons"] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["scripts/**/*.mjs"],
    languageOptions: {
      globals: {
        process: "readonly",
        fetch: "readonly",
        URL: "readonly",
        console: "readonly",
        structuredClone: "readonly",
        Buffer: "readonly"
      }
    }
  },
  {
    files: ["src/**/*.{ts,tsx}"],
    languageOptions: { parserOptions: { ecmaVersion: 2022, sourceType: "module" } },
    plugins: { "react-hooks": reactHooks, "react-refresh": reactRefresh },
    rules: {
      ...reactHooks.configs.recommended.rules,
      "react-refresh/only-export-components": ["warn", { allowConstantExport: true }]
    }
  }
];

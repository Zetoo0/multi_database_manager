import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import pluginReact from "eslint-plugin-react";
import tailwind from "eslint-plugin-tailwindcss";

export default tseslint.config(
  { ignores: ["dist"] },
  {
    files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"],

    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },

    rules: {
      "react/react-in-jsx-scope": "off",
    },

    extends: [
      js.configs.recommended,
      pluginReact.configs.flat.recommended,
      ...tseslint.configs.recommended,
      ...tailwind.configs["flat/recommended"],
    ],
  }
);

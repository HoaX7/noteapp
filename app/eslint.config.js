import typescriptPlugin from "@typescript-eslint/eslint-plugin";
import typescriptParser from "@typescript-eslint/parser";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";

export default [
  // TypeScript
  {
    files: ["**/*.ts"],
    languageOptions: {
      parser: typescriptParser,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      "@typescript-eslint": typescriptPlugin,
    },
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
      "no-mixed-spaces-and-tabs": "off",
      "@typescript-eslint/no-unused-vars": "warn",
      indent: ["error", "tab", { SwitchCase: 1 }],
      "linebreak-style": ["error", "unix"],
      quotes: ["error", "double"],
      semi: ["error", "always"],
      "max-len": ["error", { code: 120 }],
      "space-infix-ops": ["error", { int32Hint: false }],
      "space-in-parens": ["error", "never"],
      "space-before-blocks": ["error", "always"],
      "array-bracket-spacing": ["error", "always"],
      "object-curly-spacing": ["error", "always"],
      "object-curly-newline": [
        "error",
        {
          multiline: true,
          minProperties: 5,
          consistent: false,
        },
      ],
      "brace-style": ["error", "1tbs"],
      "object-property-newline": [
        "error",
        { allowMultiplePropertiesPerLine: false },
      ],
      "keyword-spacing": [
        "error",
        {
          before: true,
          after: true,
        },
      ],
      "comma-spacing": ["error", { after: true }],
      "key-spacing": [
        "error",
        {
          beforeColon: false,
          afterColon: true,
          mode: "strict",
        },
      ],
      "arrow-spacing": [
        "error",
        {
          before: true,
          after: true,
        },
      ],
    },
  },

  // Svelte
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: typescriptParser,
        extraFileExtensions: [".svelte"],
      },
    },
    plugins: {
      svelte: sveltePlugin,
      "@typescript-eslint": typescriptPlugin,
    },
    settings: {
      "import/core-modules": ["svelte"],
    },
    rules: {
      ...sveltePlugin.configs.recommended.rules,
      quotes: ["error", "double"],
      semi: "error",
      "svelte/indent": [
        "error",
        {
          indent: "tab",
          ignoredNodes: [],
          switchCase: 1,
          alignAttributesVertically: false,
        },
      ],
      "svelte/html-quotes": [
        "error",
        {
          prefer: "double", // or "single"
          dynamic: {
            quoted: false,
            avoidInvalidUnquotedInHTML: false,
          },
        },
      ],
      "svelte/html-closing-bracket-spacing": [
        "error",
        {
          startTag: "never", // or "always" or "ignore"
          endTag: "never", // or "always" or "ignore"
          selfClosingTag: "always", // or "never" or "ignore"
        },
      ],
    },
  },
];

module.exports = {
	// ...
	extends: ["plugin:svelte/recommended"],
	// ...
	parser: "@babel/eslint-parser",
	// Add an `overrides` section to add a parser configuration for svelte.
	overrides: [
		{
			files: ["*.svelte"],
			parser: "svelte-eslint-parser",
		},
		// ...
	],
	rules: {
		"no-mixed-spaces-and-tabs": "off",
		indent: ["error", "tab", { SwitchCase: 1 }],
		"linebreak-style": ["error", "unix"],
		quotes: ["error", "double"],
		semi: ["error", "always"],
		"max-len": ["error", { code: 120 }],
		"space-infix-ops": [
			"error",
			{
				int32Hint: false,
			},
		],
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
			{
				allowMultiplePropertiesPerLine: false,
			},
		],
		"keyword-spacing": [
			"error",
			{
				before: true,
				after: true,
			},
		],
		"comma-spacing": [
			"error",
			{
				after: true,
			},
		],
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
};
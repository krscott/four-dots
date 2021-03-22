module.exports = {
    env: {
        browser: true,
        es2021: true
    },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: 12,
        sourceType: "module",
        tsconfigRootDir: __dirname,
        project: ["./tsconfig.json"],
        extraFileExtensions: [".svelte"],
    },
    plugins: [
        "@typescript-eslint",
        "svelte3"
    ],
    overrides: [
        {
            files: ["*.svelte"],
            processor: "svelte3/svelte3"
        }
    ],
    rules: {
        indent: [
            "error",
            4
        ],
        "linebreak-style": [
            "error",
            "windows"
        ],
        quotes: [
            "error",
            "double"
        ],
        semi: [
            "error",
            "never"
        ]
    },
    settings: {
        "svelte3/typescript": require("typescript")
    }
}

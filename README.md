# jpm

jpm is a next-generation package management system for JavaScript, that only supports ESM, and is not coupled to the current Node.js module and npm systems, and instead is a new paradigm to move the JavaScript ecosystem forward.

## Why?

The state of Node.js, npm, and the `node_modules` folder is a complete mess, especially for package authors, but also for package consumers. With the introduction of ESM, the `.cjs` and `.mjs` file extensions, the dual-package hazard problem, and overlapping competition between npm, pnpm, and yarn, the ecosystem is in a state of chaos. This is further exacerbated by TypeScript, its insane number of compiler options, the `.cts` (`.d.cts`) and `.mts` (`.d.mts`) file extensions, and overall interoperability.

The overhead of publishing a package is far too great, and there's no solution in sight. Instead of trying to "fix" the state of things, jpm is a complete reimagining of how everything can work using modern practices and technologies.

### As a package author

Author your packages in modern ESM. Packages and their source code are published as-is, as packages are compiled on-demand for consumers. No longer are you required to pre-compile packages before publishing, nor fiddle with `package.json` configuration (especially `exports`), or worry about deep imports or the dual-package hazard problem.

### As a package consumer

When consuming packages, tailor them to your exact requirements, by requesting all packages in a specific compilation target. The days of mismatching module systems and incorrectly compiled packages are over.

```toml
target = "es2016"
```

## Learn more

In order of context and importance.

- [How it works](./docs/how-it-works)
- [Package registy](./docs/package-registry)

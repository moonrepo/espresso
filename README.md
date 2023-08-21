# Espresso

Espresso is a next-generation package registry and management system for JavaScript, that is not
coupled to the current Node.js module and npm systems, and instead is a new paradigm to move the
JavaScript ecosystem forward.

## Why?

The state of Node.js, npm, and the `node_modules` folder is a complete mess, especially for package
owners, but also for package consumers. With the introduction of ESM, the `.cjs` and `.mjs` file
extensions, the dual-package hazard problem, and competition between npm, pnpm, and yarn, the
ecosystem is in a state of chaos. This is further exacerbated By TypeScript, its insane number of
compiler options, the `.cts` (`.d.cts`) and `.mts` (`.d.mts`) file extensions, and overall
interoperation.

The overhead of publishing a package is far too great, and there's no solution in sight. Instead of
trying to "fix" the state of things, Espresso is a complete reimagining of how everything can work
in a modern world.

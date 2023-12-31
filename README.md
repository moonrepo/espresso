# espresso

espresso is a next-generation package registry and package management system for JavaScript, that is
not coupled to the current Node.js module and npm systems, and instead is a new paradigm to move the
JavaScript ecosystem forward.

> THIS PROJECT IS CURRENTLY AN EXPERIMENT AND IS HEAVILY IN DEVELOPMENT! DEFINITELY NOT PRODUCTION
> READY, AND MAY NEVER REACH FRUITION!

## Why a new registry?

The state of Node.js, npm, and the `node_modules` folder is a complete mess, especially for package
owners, but also for package consumers. With the introduction of ESM, the `.cjs` and `.mjs` file
extensions, the dual-package hazard problem, competition between npm, pnpm, and yarn, and much more,
the ecosystem is in a state of chaos. This is further exacerbated by TypeScript, its high number of
compiler options, the `.cts` (`.d.cts`) and `.mts` (`.d.mts`) file extensions, module resolution
woes, and other TS features.

The overhead of publishing a package is far too great, and there's no solution in sight. Instead of
trying to "fix" the state of things, espresso is a complete reimagining of how everything can work
in a modern world.

### As a package author

Author your packages in modern ESM. Packages and their source code are published as-is, as packages
are compiled on-demand for consumers. No longer are you required to pre-compile packages before
publishing, nor fiddle with `package.json` configuration (especially `exports`), or worry about deep
imports, or the dual-package hazard problem.

### As a package consumer

When consuming packages, tailor them to your exact requirements, by requesting all dependencies in a
specific compilation target. The days of mismatching module systems and incorrectly compiled
packages are over.

```toml
[install]
target = "es2016"
```

## Why the name espresso?

espresso is currently a codename until I can think of something better, but there are a few reasons
for it. The current binary is named `espm`, which stands for EcmaScript Package Manager, and since
the first 3 letters align with espresso, and JavaScript is already a coffee themed name, it made
sense!

But why not simply call it espm? Well, there's more than just a package manager, so using that
acronym for other services doesn't make much sense. We could also use esp, or espackage, or
something similar, but... branding?

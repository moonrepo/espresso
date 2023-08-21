# Espresso

Espresso is a next-generation package registry and management system for JavaScript, that is not
coupled to the current Node.js module and npm systems, and instead is a new paradigm to move the
JavaScript ecosystem forward.

> THIS PROJECT IS CURRENTLY AN EXPERIMENT AND IS HEAVILY IN DEVELOPMENT! DEFINITELY NOT PRODUCTION
> READY, AND MAY NEVER REACH FRUITION!

## Why a new registry?

The state of Node.js, npm, and the `node_modules` folder is a complete mess, especially for package
owners, but also for package consumers. With the introduction of ESM, the `.cjs` and `.mjs` file
extensions, the dual-package hazard problem, and competition between npm, pnpm, and yarn, the
ecosystem is in a state of chaos. This is further exacerbated By TypeScript, its insane number of
compiler options, the `.cts` (`.d.cts`) and `.mts` (`.d.mts`) file extensions.

The overhead of publishing a package is far too great, and there's no solution in sight. Instead of
trying to "fix" the state of things, Espresso is a complete reimagining of how everything can work
in a modern world.

## Why the name Espresso?

Espresso is currently a codename until I can think of something better, but there are a few reasons
for it. The current binary is named `espm`, which stands for EcmaScript Package Manager, and since
the first 3 letters align with espresso, and JavaScript is already a coffee themed name, it made
sense!

But why not simply call it espm? Well, there's more than just a package manager, so using that
acronym for other services doesn't make much sense. We could also use esp, or espackage, or
something similar, but... branding?

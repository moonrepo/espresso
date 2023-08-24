# Create a package

A package is denoted by a manifest file called `esp.toml`. This manifest is written in
[TOML](https://toml.io/en/) and configures metadata about the package, like its name and
description, as well as dependencies (other packages) it requires, and also denotes the root of the
package.

Begin by creating the `esp.toml` file in your project, and insert the following initial content:

```toml
[package]
name = "namespace/package"
```

> [Learn more about `esp.toml` in the official manifest documentation](../esp-toml.md)

## Package structure

For projects that will be published as a package to the registry, all source files must be within a
`src` folder. The `tests` folder is optional, but encouraged.

For non-published projects, like applications, the folder structure is not enforced, but is also
encouraged for consistency.

```
package/
├── src/
│   ├── **/*
│   └── index.*
├── tests/
│   └── **/*
└── esp.toml
```

### Source files

All source files that will be available to consumers must be located in the `src` folder. This
includes JavaScript, TypeScript, CSS, images, and other related files.

The entry point for the package must be named `index.js` (or `.ts`, `.tsx`, etc) and must be located
in the root of the `src/` folder.

#### JavaScript

All JavaScript (and TypeScript) code _must_ be written in strict ECMAScript. This means that all
imported code _must_ use `import` or `import()`, and any usage of `require()` or CommonJS syntax is
forbidden.

### Test files

Test files should be located in the `tests` folder, but this is not a hard requirement. If test
files are located in `src`, at minimum they should be contained in a `__tests__` folder, or end with
a `.test` file suffix.

We suggest using a distinct tests folder, as it implicitly filters out test files from the
publishing process.

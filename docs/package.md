# Package

A package is a collection of source code and metadata that can be published to a registry, and
subsequently consumed by other packages.

## Name requirements

All [package names](./esp-toml.md#package) _must be_ in alphanumeric kebab-case, composed of 2
components joined with `/` -- the namespace on the left, and package name on the right. The
namespace is similar to npm private scopes but without the `@`.

Each component must abide the following rules:

- Lowercase only.
- Start with a letter.
- End with a letter or number.
- Be between 2 and 32 characters in length.
- Can use dashes (`-`).
- Dashes cannot be consecutively repeated.

These requirements help to solve the following problems compared to npm package names:

- No "global" scope for packages.
- No more package name squatting.
- Forked packages are easily recognizable by the owner scope.
- The `namespace/package` format easily maps to repository slugs.

Example package names:

- `babel/core`
- `babel/preset-env`
- `meta/jest`
- `meta/react`
- `moonrepo/moon`
- `microsoft/typescript`

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

### Test files

Test files should be located in the `tests` folder, but this is not a hard requirement. If test
files are located in `src`, at minimum they should be contained in a `__tests__` folder, or end with
a `.test` file suffix.

We suggest using a distinct tests folder, as it implicitly filters out test files from the
publishing process.

### Info files

Info files are just that, informational. These are files like a readme, changelog, license, etc.
They should exist in the package root, and will be copied to an output directory during build.

## Create a package

A package is denoted by a manifest file called [`esp.toml`](./esp-toml.md). This manifest is written
in [TOML](https://toml.io/en/) and configures metadata about the package, like its name and
description, as well as dependencies (other packages) it requires, and also denotes the root of the
package.

Begin by creating the `esp.toml` file in your project, and insert the following initial content:

```toml
[package]
name = "namespace/package"
```

Next we'll need to create a `src` folder and optional `tests` folder, as required by the
[package structure](#package-structure). We can then populate these with files.

All JavaScript (and TypeScript) code _must_ be written in strict ECMAScript. This means that all
imported code _must_ use `import` or `import()`, and any usage of `require()` or CommonJS syntax is
forbidden.

## Build a package

Once a [package has been created](#create-a-package), and source files have been added, a package
can be built. Building automatically happens within the registry as part of the publishing process,
but a package can be built and tested manually with the [`espm build`](./commands/build.md) command.

When a package is built, all JavaScript and TypeScript files will be transformed with
[swc](https://swc.rs/), and all non-JavaScript files will be copied. The folder structure will be
preserved in the output directory.

```shell
espm build --target es2020
```

> [Learn more about selecting packages to build](./workspace.md#selecting-packages)

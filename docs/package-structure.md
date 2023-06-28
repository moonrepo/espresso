# Package structure

For projects that will be published as a package, the following folder structure is required:

```
package/
├── src/
│   ├── **/*
│   └── index.*
├── tests/
│   └── **/*
└── jpm.toml
```

### Source files

All source files that will be available to consumers must be located in the `src/` folder. This includes JavaScript, TypeScript, CSS, images, and other related files.

The entry point for the package must be named `index.js` (or `.ts`, `.tsx`, etc) and must be located in the root of the `src/` folder.

#### JavaScript

All JavaScript (and TypeScript) code _must_ be written in strict ECMAScript. This means that all imported code _must_ use `import` or `import()`, and any usage of `require()` is forbidden.

### Test files

Test files should be located in the `tests/` folder, but this is not a hard requirement. If test files are located in `src/`, at minimum they should be contained in a `__tests__` folder, or end with a `.test` file suffix.

We suggest using a distinct tests folder, as it implicitly filters out test files from the publishing process.

## Compiling

When a package is compiled, all JavaScript based files will be transformed (down-leveled to the [`target`](./project-manifest#target)), and all non-JavaScript files will be copied. Files will be written to a destination that does _not_ include the `src` or `tests` folders.

For example, say we have the following project and files.

```
package/
├── src/
│   ├── components/*.tsx
│   ├── data/*.json
│   ├── styles/*.css
│   ├── images/*.png
│   └── index.ts
├── tests/
│   └── **/*.test.ts
└── jpm.toml
```

When compiled to a [`module`](./project-manifest#module) format (like `esm`), the following files will be available for consumers.

```
package/esm/
├── components/*.{js,d.ts}
├── data/*.json
├── styles/*.css
├── images/*.png
├── index.{js,d.ts}
└── jpm.toml
```

### Optimizations

Images and other applicable files will be optimized/compressed when copied during the compilation process.

> Minification _does not_ happen during compilation, as that's a bundling feature, and primarily an application concern!

# Build a package

Once a [package has been created](./create-package.md), and source files have been added, a package
can be built. Building automatically happens within the registry as part of the publishing process,
but a package can be built and tested manually with the [`espm build`](../commands/build.md)
command.

When a package is built, all JavaScript and TypeScript files will be transformed with
[swc](https://swc.rs/), and all non-JavaScript files will be copied. The folder structure will be
preserved in the output directory.

```shell
espm build --target es2020
```

> [Learn more about selecting packages to build](../workspaces.md#selecting-packages)

## Build targets

Builds can define the compilation target using the `--target` option (for package authors), or the
`build.target` setting in [`espm.toml`](../espm-toml.md) (for package consumers). If not defined,
defaults to `es2018` (5 years old).

```toml
[build]
target = "es2020"
```

The target controls the following:

- Sets the build output directory to `.espm/<target>`.
- For JavaScript, downlevels and transforms syntax. This is achieved with swc's
  [`jsc.target`](https://swc.rs/docs/configuration/compilation#jsctarget) setting.
- For TypeScript, sets the [`target`](https://www.typescriptlang.org/tsconfig#target) and
  [`lib`](https://www.typescriptlang.org/tsconfig#lib) compiler options during
  [declaration generation](#typescript-declarations).

## Output directory

Files will be written to an `.espm/<target>` output directory, relative from the package root, and
will _not_ include the `src` folder prefix, or the `tests` folder.

For example, say we have the following package structure:

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
└── espm.toml
```

When built, the output directory will contain the following files:

```
package/.espm/es2020/
├── components/*.{mjs,d.mts}
├── data/*.json
├── styles/*.css
├── images/*.png
├── index.{mjs,d.mts}
└── espm.toml
```

## JavaScript

JavaScript files will be transformed with swc, and downleveled to the chosen target. In addition to
this process, the following occurs:

- Files are written with an `.mjs` extension to indicate they are ESM only.
- Imports and exports within the file are suffixed with the `.mjs` extension.
- CommonJS syntax (`require`, `__dirname`, etc) will throw an error during build.

## TypeScript

If a TypeScript file is detected in `src`, we'll automatically run a `tsc` process to generate
declarations in the output directory. We achieve this with the following steps:

- Create an `.espm/tsconfig.<target>.json` file, pre-configured for the chosen `target`. Will also
  set the correct `module`, `rootDir`, `outDir`, so on and so forth.
- Run `tsc` with the above configuration file, in the package root.
- If successful, rename all `.d.ts` files to `.d.mts`. We do this since we're ESM only, and
  JavaScript files are built with the `.mjs` extension.

To demonstrate this, say we have the following source:

```
package/
├── src/
│   └── index.ts
└── espm.toml
```

When built, the output directory will contain an `.mjs` file and `.d.mts` declaration file.

```
package/.espm/es2020/
├── index.d.mts
├── index.mjs
└── espm.toml
```

### Custom `tsconfig.json`

Our generated `tsconfig.<target>.json` uses sane defaults and should work for most use cases, but
for situations where they fail to compile, or the compiler options need to be customized, you can
provide your own `tsconfig.espm.json` file in the package root.

Since we need to support multiple targets, this file will be copied to
`.espm/tsconfig.<target>.json` and modified accordingly.

## Assets

As mentioned previously, all non-JavaScript files in `src` are known as assets, and are
automatically copied to the output directory. An asset is anything from an image, to audio, to SVG,
or even JSON.

### Optimizations

Images and other applicable files will be optimized/compressed when copied. At this point in time,
only `png` files are supported.

This can be customized in [`espm.toml`](../espm-toml.md).

```toml
[build]
# Turn off optimizations
optimize-png = false
# Customize optimization level (default 2)
optimize-png = 6
```

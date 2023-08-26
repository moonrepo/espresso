# `build`

The `espm build` command can be used to build a package for a chosen compilation target.

```shell
espm build -p namespace/package
espm build -p namespace/package -t es2020
```

> This command is used by the registry's [build-on-demand service](../registry.md#build-on-demand)
> but exists within `espm` so that packages can be tested and built locally.

## Options

- Inherits all [package selection options](../workspace.md#selecting-packages).
- `--target`, `-t` - Target to transform and downlevel JavaScript code to.
  - Accepts `es2015` through `es2022`.
  - Defaults to `es2018`.
  - Can be defined with `ESPM_TARGET` environment variable.

## Build targets

Builds can define the compilation target using the `--target` option (for package authors), or the
`install.target` setting in [`esp.toml`](../esp-toml.md#install) (for package consumers).

```toml
[install]
target = "es2020"
```

The target controls the following:

- Sets the build output directory to `.espm/<target>`.
- For JavaScript, downlevels and transforms syntax. This is achieved with swc's
  [`jsc.target`](https://swc.rs/docs/configuration/compilation#jsctarget) setting.
- For TypeScript, sets the [`target`](https://www.typescriptlang.org/tsconfig#target) and
  [`lib`](https://www.typescriptlang.org/tsconfig#lib) compiler options during
  [declaration generation](#typescript).

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
└── esp.toml
```

When built, the output directory will contain the following files:

```
package/.espm/es2020/
├── components/*.{mjs,d.mts}
├── data/*.json
├── styles/*.css
├── images/*.png
├── index.{mjs,d.mts}
└── esp.toml
```

## Sources

### JavaScript

JavaScript (and TypeScript) files will be transformed with [swc](https://swc.rs/), and downleveled
to the chosen target. In addition to this process, the following occurs:

- Files are written with an `.mjs` extension to indicate they are ESM only.
- Imports and exports within the file are suffixed with the `.mjs` extension.
- CommonJS syntax (`require`, `__dirname`, etc) will throw an error during build.

### TypeScript

If a TypeScript file is detected in `src`, we'll automatically run a `tsc` process to generate
declarations in the output directory. We achieve this with the following steps:

- Creates an `.espm/tsconfig.<target>.json` file, pre-configured for the chosen `target`. Will also
  set the correct `module`, `rootDir`, `outDir`, so on and so forth.
- Runs `tsc` with the above configuration file, in the package root.
- If successful, renames all `.d.ts` files to `.d.mts`. We do this since we're ESM only, and
  JavaScript files are built with the `.mjs` extension.

To demonstrate this, say we have the following source:

```
package/
├── src/
│   └── index.ts
└── esp.toml
```

When built, the output directory will contain an `.mjs` file and a `.d.mts` declaration file.

```
package/.espm/es2020/
├── index.d.mts
├── index.mjs
└── esp.toml
```

#### Custom `tsconfig.json`

Our generated `tsconfig.<target>.json` uses sane defaults and should work for most use cases, but
for situations where they fail to compile, or the compiler options need to be customized, you can
provide your own `tsconfig.espm.json` file in the package root.

Since we need to support multiple targets, this file will be copied to
`.espm/tsconfig.<target>.json` and modified accordingly.

### Assets

As mentioned previously, all non-JavaScript files in `src` are known as assets, and are
automatically copied to the output directory. An asset is anything from an image, to audio, to SVG,
or even JSON.

#### Optimizations

Images and other applicable files will be optimized/compressed when copied. At this point in time,
only `png` files are supported.

This can be customized in [`esp.toml`](../esp-toml.md#build).

```toml
[build]
# Turn off optimizations
optimize-png = false
# Customize optimization level (default 2)
optimize-png = 6
```

## FAQ

#### Why is `esnext` not a supported target?

ESNext is a what we call a moving target and may contain experimental or unstable features. We do
not want to encourage the use of such features in production code, especially code that is provided
to consumers.

#### Why is `es3` and `es5` not supported targets?

We want to only support modern code and these are far too legacy. ES2015 is 8 years old, which is
more than enough of a standard timeframe to support.

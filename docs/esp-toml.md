# `esp.toml`

The `esp.toml` manifest is a configuration file at the root of a package or workspace that defines
metadata, dependencies, and more.

> ESP stands for Ecma Script Package.

## `[workspace]`

For a multi-package workspace, defines workspace metadata and how to find packages. This setting
_must only_ exist in the root manifest, relative from the lockfile.

For a single package workspace, this setting _must not_ be used, and [`[package]`](#package) should
be used instead.

Supports the following fields:

- `packages` (string[]) - List of relative globs to find packages.

```toml
[workspace]
packages = ["apps/*", "packages/*"]
```

## `[package]`

Defines package metadata and supports the following fields:

- `name` (string) - [Name of the package](./package.md#name-requirements), including namespace.
- `version` (string) - Current version.
- `description` (string) - Short description of the package.
- `keywords` (string[]) - List of keywords.
- `license` (string) - License identifier in SPDX format.
- `repository` (string) - URL to the repository. Must be a valid cloneable Git URL.
- `homepage` (string) - URL to the homepage.
- `documentation` (string) - URL to the documentation.
- `publish` (bool) - Whether to publish the package or not.

```toml
[package]
name = "espresso/cli"
version = "1.2.3"
description = "Espresso package manager."
license = "MIT"
```

## `[build]`

Controls how a package is built with [`espm build`](./commands/build.md).

Supports the following fields:

- `decorators` (legacy) - Enables decorators with the chosen preset. Defaults to `null`.
- `exclude` (string[]) - List of file globs, relative from `src`, to exclude from the build.
  Defaults to empty list.
- `optimize-png` (bool | number) - Optimizes `.png` assets during build. Can customize compression
  level. Defaults to `true` (level 2).

```toml
[build]
decorators = "legacy"
optimize-png = 4
```

## `[dependencies]`

Maps dependencies (packages in the registry) that this package or workspace depends on. The map key
is the package name, while the value is a
[semantic version requirement](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).

```toml
# These are example packages!
[dependencies]
"meta/react" = "18.0.0"
"vercel/next" = "~1.13.0"
```

> Lack of a requirement symbol is equivalent to `^`, for example, `1.2.3` and `^1.2.3` are the same.

## `[dev-dependencies]`

Like [`[dependencies]`](#dependencies) but only used for development. These dependencies are not
installed when `espm install` is ran in production-only mode.

```toml
# These are example packages!
[dev-dependencies]
"microsoft/typescript" = "=5.2.2"
"prettier/cli" = "3.0.0"
```

## `[install]`

Controls how dependencies are installed with `espm install`.

Supports the following fields:

- `target` (es20xx) - ECMAScript target to request/compile dependencies to. Defaults to `es2018`.

```toml
[install]
target = "es2020"
```

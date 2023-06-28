# Project manifest

The manifest is a configuration file at the root of a project that defines metadata, dependencies, and more.

The manifest is a TOML file named `jpm.toml`. Name and format TBD!

## `dependencies`

External packages required for the project to operate. Will be downloaded and linked during `jpm install`.

```toml
[dependencies]
"owner/package" = "^1.0.0"
```

## `dev-dependencies`

Like [`dependencies`](#dependencies), but will not be downloaded or linked in production-only mode.

## `target`

The ECMAScript specification to target when compiling packages for the current project. Will recursively be applied to all dependencies.

```toml
target = "es2022"
```

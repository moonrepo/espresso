# Documentation

Install the latest version of espresso with the following commands
([or download manually](https://github.com/moonrepo/espresso/releases/latest)):

```shell
# Linux, macOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/moonrepo/espresso/releases/latest/download/espresso_cli-installer.sh  | sh

# Windows
irm https://github.com/moonrepo/espresso/releases/latest/download/espresso_cli-installer.ps1 | iex
```

This will install the binary to `~/.espresso/bin` and add that directory to your `PATH`.

## Concepts

- [Package](./package.md) - A collection of source code and metadata.
- [Workspace](./workspace.md) - Manages one or many packages within a repository.
- [Registry](./registry.md) - Stores packages in a remote service.

## Configs

- [`esp.toml`](./esp-toml.md) - Package manifest file.
- [`espm.lock`](./espm-lock.md) - Dependencies lockfile.

## Commands

- [`espm init`](./commands/init.md) - Initialize a workspace.
- [`espm new`](./commands/new.md) - Create a package.
- [`espm build`](./commands/build.md) - Build a package.

## Terminology

- espresso - Umbrella name for this project.
- esp - Ecma Script Package.
- espm - Ecma Script Package Manager.
- espr, esper - Ecma Script Package Registry.

# Workspace

A workspace contains one package (polyrepo) or multiple packages (monorepo), and is the foundation
for managing dependencies, interacting with the store, running `espm` commands, and more.

> The workspace is typically at the root of a repository, but can be located anywhere.

## Root detection

The root of a workspace is detected using the following rules:

- An [`espm.lock`](./espm-lock.md) file.
- An [`esp.toml`](./esp-toml.md#workspace) file with a `[workspace]` section (monorepo).
- An [`esp.toml`](./esp-toml.md#package) file with a `[package]` section (polyrepo).

## Creating a workspace

A workspace can be initialized using the [`espm init`](./commands/init.md) command.

```shell
espm init
```

### Polyrepo

A polyrepo is a workspace with a single package, and is commonly implemented in a "one package per
repository" pattern.

An example of the repository file structure and root [`esp.toml`](./esp-toml.md) would look like the
following:

```
/
├── src/
├── tests/
├── esp.toml
├── espm.lock
├── LICENSE
└── README.md
```

```toml
[package]
name = "poly/repo"
```

### Monorepo

A monorepo is a workspace that contains multiple packages, and will resolve, install, and flatten
dependencies in unison. A monorepo is commonly implemented in a "many packages per repository"
pattern.

An example of the repository file structure and root [`esp.toml`](./esp-toml.md) would look like the
following:

```
/
├── packages/
│   ├── foo/
│   |   ├── src/
│   |   ├── esp.toml
│   |   ├── LICENSE
│   |   └── README.md
│   ├── bar/
│   |   ├── src/
│   |   ├── esp.toml
│   |   ├── LICENSE
│   |   └── README.md
│   └── baz/
│       ├── src/
│       ├── esp.toml
│       ├── LICENSE
│       └── README.md
├── esp.toml
├── espm.lock
├── LICENSE
└── README.md
```

```toml
[workspace]
packages = ["packages/*"]
```

## Selecting packages

If a polyrepo, the primary package is always automatically selected. The rest of this section can be
skipped if you're using a polyrepo.

If a monorepo, one, many, or all packages can be selected with the following CLI options. These
options are available for all `espm` commands.

### `--workspace`

The `--workspace` (`-w`) option will select _all_ packages within the workspace.

```shell
espm build --workspace
```

### `--package`

The `--package` (`-p`) option will select a package by name, and can be provided multiple times.

```shell
espm build -p namespace/one -p namespace/two -p namespace/three
```

### `--filter`

The `--filter` (`-f`) option will select all package names that match a glob, and can be provided
multiple times.

Since package names contain a forward slash (`/`), the glob matching considers the name as 2
distinct folders, and the provided pattern should represent this.

```shell
# Select all
espm build --filter '*/*'

# Select all in a namespace
espm build --filter 'espresso/*'
espm build --filter 'es{presso,modules}/*'

# Select by name in any namespace
espm build --filter '*/components' --filter '*/utils'

# Select by name and namespace
espm build --filter 'ui-*/react-*'
```

Negated globs can also be provided, allowing you to select all matching packages, but exclude some
from the final result.

```shell
espm build -f 'ui/*' -f '!ui/styles' -f '!ui/images'
```

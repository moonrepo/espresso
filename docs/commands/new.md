# `new`

The `espm new` command can be used to create and scaffold a new package. It will prompt you for a
package name, description, and other metadata, and will create a package with the necessary files,
like [`esp.toml`](../esp-toml.md).

```shell
espm new
espm new --name namespace/package --yes
```

> Prompt values can be autopopulated by passing options on the command line.

## Options

- `--to` - Destination to create the package in, relative from the current working directory.
  Defaults to `.`.
- `--name`, `-n` - Name of package.
- `--description`, `-d` - Description of package.
- `--keyword`, `-k` - List of keywords about the package. Can be provided multiple times.
- `--yes` - Skip all interactive prompts and use default or provided values.

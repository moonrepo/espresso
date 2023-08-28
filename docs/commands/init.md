# `init`

The `espm init` command can be used to create a new Espresso [workspace](../workspace.md) for a
polyrepo or monorepo setup. It will launch an interactive prompt, gathering information necessary
for the workspace, and creating necessary files like [`esp.toml`](../esp-toml.md).

```shell
espm init
```

> Prompt values can be autopopulated by passing options on the command line.

## Options

- `--to` - Destination to create the workspace in, relative from the current working directory.
  Defaults to ".".
- `--yes` - Skip all interactive prompts and use default or provided values.

These options can be used to populate information about a package when initializing a polyrepo.

- `--name`, `-n` - Name of package.
- `--description`, `-d` - Description of package.
- `--keyword`, `-k` - List of keywords about the package. Can be provided multiple times.

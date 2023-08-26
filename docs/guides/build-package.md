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

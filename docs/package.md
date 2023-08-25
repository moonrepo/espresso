# Package

A package is a collection of source code and metadata that can be published to a registry, and
subsequently consumed by other packages.

## Name requirements

All [package names](./esp-toml.md#package) _must be_ in alphanumeric kebab-case, composed of 2
components joined with `/` -- the namespace on the left, and package name on the right. The
namespace is similar to npm private scopes but without the `@`.

Each component must abide the following rules:

- Lowercase only.
- Start with a letter.
- End with a letter or number.
- Be between 2 and 32 characters in length.
- Can use dashes (`-`).
- Dashes cannot be consecutively repeated.

These requirements help to solve the following problems compared to npm package names:

- No "global" scope for packages.
- No more package name squatting.
- Forked packages are easily recognizable by the owner scope.
- The `namespace/package` format easily maps to repository slugs.

Example package names:

- `babel/core`
- `babel/preset-env`
- `meta/jest`
- `meta/react`
- `moonrepo/moon`
- `microsoft/typescript`

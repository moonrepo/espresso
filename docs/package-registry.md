# Package registry

The registry is an upstream service (the server) that stores records of all available packages, their versions, supported platforms and module formats, and stores their compiled tarballs (in cloud storage).

With that said, jpm will have a completely separate registry than npm, that works quite differently. This is required to solve an array of problems with the current ecosystem, and these 2 registries are not backwards compatible.

## Package names

All package names must be in alphanumeric kebab-case, and will be scoped to an owner in the format of `owner/package`. This is similar to npm private scopes, but without the `@`.

These changes help to solve the following problems:

- No "global" scope for packages.
- No more package name squatting.
- Forked packages are easily recognizable by the owner scope.
- The `owner/package` format easily maps to repository slugs.

Example package names:

- `babel/core`
- `babel/preset-env`
- `meta/jest`
- `meta/react`
- `moonrepo/cli`
- `typescript/core`

## Package source code

Unlike other registries, the jpm registry will _not_ store package source code. Instead, it will store a reference to the package's git repository, and clone the repository when necessary (primarily for [compilation](#compile-on-demand)).

Packages in the registry will be linked to individual repositories using GitHub, GitLab, and Bitbucket OAuth.

The benefits of this are:

- We don't incur storage costs for source code, only compiled code (tarballs).
- We avoid storing proprietary source code for private companies.
- Utilizing OAuth provides a layer of security and authentication.

## Supported platforms

Each package can denote which platforms the package can be consumed/run on. If nothing denoted, we can attempt to detect it, or assume it can run on any platform. The following platforms are supported:

- `browser` - Any browser engine.
- `runtime` - Node/Bun.
- `deno` - Deno runtime (TBD???)
- `electron` - Electron.

Why is this necessary? A few reasons:

- When searching the registry, being able to filter results helps to find what you need.
- Consumers can request packages that only support their platform, and avoid downloading unnecessary packages.
- Certain runtimes require specific types of imports:
  - Node/Bun can use the `node:` special import.
  - Deno uses git URLs.
- For TypeScript declarations, will help to control which compiler options are required.

## Supported module formats

In jpm, only ESM source code is allowed, and only ESM compiled code will be produced. We require that all packages are written in native ESM, and during the publishing process, we'll run some verification checks to ensure that the package is valid ESM.

For example, we'll check file extensions.

- `.js`, `.jsx`, `.ts`, `.tsx` - Possibly allowed, will scan contents.
- `.cjs`, `.cts` - Not allowed.
- `.mjs`, `.mts` - Allowed.

And then check the source code itself.

- `require()`, `__filename`, `__dirname`, etc - Not allowed.

## Compile on demand

The biggest selling point of the jpm registry is that packages are compiled on demand when requested (being downloaded as a dependency). The registry is basically a compilation-as-a-service (CaaS) platform.

If a compiled package already exists, the client will download the tarball from cloud storage, and unpack it on the client's machine.

If a package has not been compiled, a background server agent will be spawned that will clone the package's repository, run the compilation process, and upload the tarball to cloud storage. The agent will then notify the registry that the package has been compiled, and the registry will then serve the tarball to the client.

The benefits of this are:

- We don't incur storage costs upfront. Only store packages that clients are actually using.
- Package authors no longer need to pre-compile their packages, fiddle with configurations, or structure `package.json` correctly.

### Caveats

- **What if GitHub or the VCS provider is down or inaccessible?**

  Not sure... simply blocked?

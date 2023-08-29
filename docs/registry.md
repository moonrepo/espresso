# Registry

The registry is an upstream service (the server) that stores records of all available packages,
their versions, supported platforms and module formats, and stores their compiled tarballs (in cloud
storage).

With that said, espresso will have a completely separate registry than npm, that works quite
differently. This is required to solve an array of problems with the current ecosystem, and these 2
registries are not backwards compatible.

## Package source code

Unlike other registries, the registry will _not_ store package source code. Instead, it will store a
reference to the package's Git repository, and clone the repository when necessary (primarily for
[compilation](#build-on-demand)).

Packages in the registry will be linked to individual repositories using GitHub, GitLab, and
Bitbucket OAuth.

The benefits of this are:

- We don't incur storage costs for source code, only compiled code (tarballs).
- We avoid storing proprietary source code for private companies.
- Utilizing OAuth provides a layer of security and authentication.

## Supported platforms

Each package will denote which platforms the package can be consumed/run on. We'll attempt to detect
it during the build process, or assume it can run on any platform. The following platforms are
supported:

- `browser` - Any browser engine.
- `bun` - Bun using `Bun` or `bun:` APIs.
- `bun-node` - Bun running against Node.js code.
- `node` - Node.js.
- `deno` - Deno.
- `electron` - Electron.

Why is this necessary? A few reasons:

- When searching the registry, being able to filter packages helps to find what you need.
- Consumers can request packages that only support their platform, and avoid downloading unnecessary
  packages.
- Certain runtimes require specific types of imports:
  - Node/Bun can use the `node:` special import.
  - Deno uses git URLs.
  - So on and so forth.
- For TypeScript declarations, will help to control which compiler options are required.

## Supported module formats

In espresso, only ESM source code is allowed, and only ESM compiled code will be produced. We
require that all packages are written in native/strict ESM, and during the publishing process, we'll
run some verification checks to ensure that the package is valid ESM.

For example, we'll check file extensions.

- `.js`, `.jsx`, `.ts`, `.tsx` - Possibly allowed, will scan contents.
- `.cjs`, `.cts` - Not allowed.
- `.mjs`, `.mts` - Allowed.

And then check the source code itself.

- `require()`, `__filename`, `__dirname`, etc - Not allowed.

## Build-on-demand

The biggest selling point of the espresso registry is that packages are built on demand when
requested (being downloaded as a dependency). The registry is basically a build-as-a-service (BaaS)
platform.

If a built package already exists, the client will download the tarball from cloud storage, and
unpack it on the client's machine.

If a package has not been built, a background job will be spawned that will clone the package's
repository, run the build process, and upload the tarball to cloud storage. The agent will then
notify the registry that the package has been compiled, and the registry will then serve the tarball
to the client.

The benefits of this are:

- We don't incur storage costs upfront. Only store packages that clients are actually using.
- Package authors no longer need to pre-build their packages, fiddle with configurations, or
  structure `package.json` correctly.

### Pre-builts

With that being said, we will support pre-built packages in the future. This will bypass the build
targets (`target`) and force consumers into whatever target the pre-built is in.

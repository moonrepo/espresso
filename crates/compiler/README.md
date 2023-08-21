# espresso_compiler

## Features

- Transforms to specific ESM targets (es2018, es2022, etc).
- ESM only source code. Will error when encountering CJS syntax.
- Appends `.mjs` to all import/export paths when transforming.

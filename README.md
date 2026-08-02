<p align="center">
  <img
    src="./images/crr.png"
    width="720"
    alt="cr-sqlite replicas converging through CRR changes">
</p>

<h1 align="center">cr-sqlite</h1>

<p align="center">
  Cargo-native conflict-free replicated relations for SQLite
</p>

<p align="center">
  <a href="https://github.com/shayne-fletcher/cr-sqlite/actions/workflows/ci.yml">
    <img src="https://github.com/shayne-fletcher/cr-sqlite/actions/workflows/ci.yml/badge.svg" alt="rust ci">
  </a>
  <a href="https://shayne-fletcher.github.io/cr-sqlite/">
    <img src="https://img.shields.io/badge/docs-github.io-blue" alt="docs">
  </a>
</p>

This repository is a fork of
[`vlcn-io/cr-sqlite`](https://github.com/vlcn-io/cr-sqlite) focused on building
its SQLite loadable extension with a modern, Cargo-native Rust toolchain.

The CRR implementation and its internal Cargo packages come from upstream.  In
this fork, the top-level `cr-sqlite` package builds those packages together
with the existing C sources into the `crsqlite` dynamic library.

For the project overview, SQL API, design, and original build instructions,
see the [upstream README](https://github.com/vlcn-io/cr-sqlite#readme) and
[documentation](https://vlcn.io/docs).  Rust API documentation for this fork
is published at
[shayne-fletcher.github.io/cr-sqlite][rustdoc].

[rustdoc]: https://shayne-fletcher.github.io/cr-sqlite/

## Build

The repository pins its Rust toolchain.  Clone it with its submodule and build
the extension from the repository root:

```sh
git clone --recurse-submodules \
  https://github.com/shayne-fletcher/cr-sqlite.git
cd cr-sqlite
cargo build -p cr-sqlite --release
```

Cargo writes the platform-specific library under `target/release`.

## Cargo dependency

Applications can build the extension directly from a pinned Git revision with
Cargo's nightly artifact-dependency support:

```toml
[dependencies.crsqlite]
package = "cr-sqlite"
git = "https://github.com/shayne-fletcher/cr-sqlite.git"
rev = "<pinned-commit>"
artifact = "cdylib"
```

Enable artifact dependencies in the consuming project:

```toml
# .cargo/config.toml
[unstable]
bindeps = true
```

Cargo makes the built library path available while compiling the consumer as
`CARGO_CDYLIB_FILE_CRSQLITE_crsqlite`.  Pass that path to SQLite or libSQL when
loading the extension.

## Development

See [DEVELOPING.md](DEVELOPING.md) for the source layout, upstream remotes, and
the procedure for reconstructing and building an exact pinned revision.

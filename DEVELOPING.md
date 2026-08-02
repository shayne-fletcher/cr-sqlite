# Developing cr-sqlite

This fork is assembled from two repositories:

- `cr-sqlite`: <https://github.com/shayne-fletcher/cr-sqlite>
- `sqlite-rs-embedded`:
  <https://github.com/shayne-fletcher/sqlite-rs-embedded>

The latter is checked in as the `core/rs/sqlite-rs-embedded` submodule.  Every
cr-sqlite commit records the exact sqlite-rs commit it uses, so a pinned
cr-sqlite revision identifies the complete source graph.

## Reconstruct a pinned revision

Set `revision` to the cr-sqlite commit recorded by the consuming project.  For
a Cargo Git dependency, this is normally the commit in `rev` or `Cargo.lock`.

```sh
revision=<pinned-cr-sqlite-commit>
checkout_dir=$(mktemp -d)

git clone --no-checkout \
  https://github.com/shayne-fletcher/cr-sqlite.git \
  "$checkout_dir/cr-sqlite"

git -C "$checkout_dir/cr-sqlite" checkout --detach "$revision"
git -C "$checkout_dir/cr-sqlite" submodule sync --recursive
git -C "$checkout_dir/cr-sqlite" submodule update --init --recursive
```

Verify the parent revision and display the submodule revisions before
building:

```sh
test "$(git -C "$checkout_dir/cr-sqlite" rev-parse HEAD)" = "$revision"
git -C "$checkout_dir/cr-sqlite" submodule status --recursive

(
  cd "$checkout_dir/cr-sqlite"
  cargo build -p cr-sqlite --release --locked
)
```

No adjacent checkout is required.  The public HTTPS submodule URL and the
gitlink stored in the parent commit provide the sqlite-rs source.

## Prepare a development checkout

Clone the fork recursively and register the upstream repository:

```sh
git clone --recurse-submodules \
  https://github.com/shayne-fletcher/cr-sqlite.git
cd cr-sqlite
git remote add upstream https://github.com/vlcn-io/cr-sqlite.git
```

The sqlite-rs fork carries its modern Rust work on `modern-rust`.  To work on
that repository rather than merely consume its pinned commit:

```sh
git -C core/rs/sqlite-rs-embedded remote add upstream \
  https://github.com/vlcn-io/sqlite-rs-embedded.git
git -C core/rs/sqlite-rs-embedded fetch origin modern-rust
git -C core/rs/sqlite-rs-embedded switch --track origin/modern-rust
```

Commit and push sqlite-rs changes first.  Then commit the resulting submodule
gitlink change in cr-sqlite.  Consumers should continue to pin a cr-sqlite
commit rather than depend on either repository's moving branch name.

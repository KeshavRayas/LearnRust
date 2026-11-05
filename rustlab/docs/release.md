# Release

## Versioning

Use semantic versioning. The content and the CLI share a version until the content format changes.

## Before a release

Run from `rustlab/`:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask validate
cargo xtask e2e
```

All five commands must pass on Windows, macOS, and Linux.

## Build artifacts

Build the CLI in release mode:

```text
cargo build --release -p rustlab-cli
```

Package the binary with the `content/` directory next to it:

```text
rustlab/
  rustlab.exe
  content/
```

The CLI discovers `content/` next to the binary, or uses `RUSTLAB_CONTENT`.

## Smoke test

1. Unpack the artifact.
2. Run `rustlab doctor`.
3. Run `rustlab start unit-converter --dir <temp>`.
4. Copy a solution over the project.
5. Run `rustlab check`.
6. Confirm the check passes and progress is recorded.

## Changelog

Keep a `CHANGELOG.md` with sections for added, changed, and fixed. Mention content changes separately from code changes.

## Rollback

Keep the previous artifact and content bundle. A release is safe to roll back because progress is a local JSON file and the project marker is versioned.

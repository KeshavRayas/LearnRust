# Contributing

## Prerequisites

- Rust stable
- A linker for your platform. On Windows, install the Visual Studio C++ build tools.
- Git

## Setup

```text
cd rustlab
cargo test --workspace
cargo run -p rustlab-cli -- doctor
```

Run the full gate before opening a pull request:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask validate
cargo xtask e2e
```

## Worktrees and branches

Use one worktree per thread or task. Branch names follow `ws/<name>` for planned workstreams. The coordination rules live in [../docs/build-plan/coordination.md](../docs/build-plan/coordination.md).

## Adding a project

Read [docs/content-authoring.md](docs/content-authoring.md). A project is a normal Cargo package plus a manifest, lessons, tasks, hints, hidden tests, and solutions.

## Pull requests

- One workstream per pull request.
- Include tests for new behavior.
- Keep the change focused.
- Do not edit `crates/rustlab-core` without a contract change.
- Do not commit `target/` or generated files.

## Code style

- Run `cargo fmt`.
- Keep public functions small and named for what they do.
- Prefer explicit errors over `unwrap` on user input.
- Add a test when a bug is fixed.

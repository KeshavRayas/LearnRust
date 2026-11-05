# WS0: Foundation

Agent: integrator
Branch: `ws/foundation`
Wave: 0, serial gate
Depends on: nothing
Merge order: 1

## Goal

Create the product workspace, freeze the core types, and give every later workstream a compiling skeleton. Nothing else starts until this merges.

## Owns

- `rustlab/Cargo.toml`
- `rustlab/Cargo.lock`
- `rustlab/rust-toolchain.toml`
- `rustlab/crates/*/Cargo.toml` (initial versions only)
- `rustlab/crates/rustlab-core/**`
- `rustlab/xtask/Cargo.toml`, `rustlab/xtask/src/main.rs` (skeleton)
- `rustlab/.cargo/config.toml`
- `.github/workflows/rustlab-ci.yml` (initial skeleton; E2E/CI owns it after WS0)
- `rustlab/fixtures/hello-cli/**` (initial version; E2E/CI owns it after WS0)
- `docs/build-plan/status.md` (template)

## Must not touch

- The existing root `Cargo.toml`, `src/`, `examples/`, `RUST_NOTES.md`, or `README.md`
- Any product crate other than the initial skeleton
- `rustlab/content/` beyond an empty directory placeholder

## Steps

1. Create `rustlab/` as a separate Cargo workspace. Do not convert the existing `learn_rust` package into a workspace member.
2. Add members:

```toml
[workspace]
resolver = "2"
members = [
  "crates/rustlab-core",
  "crates/rustlab-content",
  "crates/rustlab-check",
  "crates/rustlab-runner",
  "crates/rustlab-explain",
  "crates/rustlab-cli",
  "xtask",
  "fixtures/hello-cli",
]
exclude = ["content/**"]
```

The content starter projects are standalone Cargo packages. Exclude `content/**` so Cargo does not try to include them in the workspace.

3. Add a `rust-toolchain.toml` pinned to stable with `rustfmt` and `clippy`.
4. Create each crate skeleton:
   - `rustlab-core`: library, no workspace dependencies
   - `rustlab-content`: library, depends on core
   - `rustlab-check`: library, depends on core
   - `rustlab-runner`: library, depends on core
   - `rustlab-explain`: library, depends on core
   - `rustlab-cli`: binary named `rustlab`, depends on all
   - `xtask`: binary named `xtask`, depends on content, check, runner, core
5. Add shared dependencies to `[workspace.dependencies]`: `anyhow`, `serde`, `serde_json`, `toml`, `thiserror`, `regex`, `uuid`, `time`, `dirs`, `clap`, `tempfile`, `assert_cmd`, `predicates`, `tracing`, `tracing-subscriber`.
6. Implement `rustlab-core` exactly as specified in [../contracts.md](../contracts.md). Keep it in modules if the file grows, but do not add I/O. Include the `ContentProvider` and `ProcessRunner` traits.
7. Create public API stubs in the other crates with the signatures from the contracts. They may return `todo!()` or `anyhow::bail!`. The goal is that `cargo check --workspace` passes and dependent crates can compile before their implementations land.
8. Add core tests:
   - `CheckReport` round-trips through JSON.
   - A sample `project.toml` parses, including `stdout = { kind = "contains", value = "6.21" }`.
   - `CheckSpec.timeout_secs` defaults to 30.
   - `Matcher::Empty` is the default.
9. Create `rustlab/.cargo/config.toml` with the xtask alias:

```toml
[alias]
xtask = "run --package xtask --"
```

10. Create `rustlab/fixtures/hello-cli`: a tiny package with one binary that prints `hello` and one passing test. This is the runner's fixture.
11. Create a minimal `.github/workflows/rustlab-ci.yml` at the repository root. Set `working-directory: rustlab` on the Cargo steps. The E2E/CI agent replaces it in WS6.
12. Create `docs/build-plan/status.md` with the table from [../coordination.md](../coordination.md).
13. Run the acceptance commands from `rustlab/`.

## Acceptance

From `rustlab/`:

```text
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p rustlab-cli -- --help
cargo run -p xtask -- --help
cargo xtask --help
```

All must pass. The CLI and xtask help output can be placeholders.

## Deliverable

A PR titled `foundation: workspace, core contracts, and fixture`.

The PR description must include the output of the acceptance commands and a note that `rustlab-core` is frozen after merge.

## Handoff

After merge, post in the integrator thread:

- The merge commit SHA.
- The list of crate names and paths.
- Any dependency that later agents should use from `[workspace.dependencies]`.
- The exact command to run the core tests.

Then start Wave 1 threads.

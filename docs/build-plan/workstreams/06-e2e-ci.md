# WS6: E2E and CI

Agent: E2E/CI
Branch: `ws/e2e-ci`
Wave: 1, harness against a fake CLI; wire in Wave 2
Depends on: WS0, then WS4
Merge order: 7

## Goal

Make the release gate real. `cargo xtask validate` checks the content. `cargo xtask e2e` checks the learner experience from the outside. CI runs both on Windows, macOS, and Linux.

## Owns

- `rustlab/xtask/**`
- `.github/workflows/rustlab-ci.yml`
- `rustlab/fixtures/**`

## Must not touch

- `rustlab/crates/**`
- `rustlab/content/tracks/**`
- `rustlab/content/errors.json`

## Depends on

- WS0 fixture and workspace.
- WS4 CLI for the black-box E2E.
- WS1, WS2, WS3 for the content validator.

Start in Wave 1 with a fake `rustlab` binary and a fixture content directory. Wire the real CLI in Wave 2.

## Commands

```text
cargo xtask validate
cargo xtask e2e [--project <id>] [--milestone <id>]
```

## `xtask validate`

For every project and milestone:

1. Copy the starter into a fresh temp directory.
2. Copy `solutions/<milestone>/` over it.
3. Copy `tests/hidden/<test>.rs` into `tests/` as the check expects.
4. Run every check with `rustlab-check` and `rustlab-runner`. Assert all pass.
5. Copy the starter into a second temp directory.
6. Copy the hidden tests in.
7. Run every check. Assert at least one fails.
8. If `broken/<milestone>/` exists, copy it over a third temp directory, run the checks, and assert the expected check fails.

Report a table:

```text
project              milestone   solution   starter   broken
unit-converter       parse       pass       fail      pass
unit-converter       convert     pass       fail      n/a
```

Exit 1 on any unexpected result. Include the failing check id and a short output excerpt.

The validator must not modify the real content tree. Every run uses `tempfile::tempdir()`.

## `xtask e2e`

Black-box test of the CLI:

1. Build `rustlab` once.
2. Create a temp directory.
3. Set `RUSTLAB_HOME=<temp>/home` and `RUSTLAB_CONTENT=<repo>/rustlab/content`.
4. Run `rustlab start unit-converter --dir <temp>/project`.
5. For each milestone:
   - Copy `solutions/<milestone>/` over the project.
   - Run `rustlab check --json`.
   - Assert exit 0 and `passed: true` in the report.
   - Read `state.json` and assert the milestone key is in `completed`.
   - Copy `broken/<milestone>/` over the project.
   - Run `rustlab check --json`.
   - Assert exit 1 and the expected check id is in the failed results.
   - Restore the solution before the next milestone.

Do not copy hidden tests in the E2E. The CLI injects them from the content project's `tests/hidden/` directory and removes them after each check.

Use `assert_cmd::cargo::cargo_bin("rustlab")` so the test runs the built binary, not an installed one.

Set `CARGO_NET_OFFLINE=true` for the learner project commands. The unit converter has no external dependencies.

No network. No timing assertions. Every run isolated by `RUSTLAB_HOME`.

## CI

Matrix: `windows-latest`, `macos-latest`, `ubuntu-latest`. Stable Rust.

Steps:

1. Checkout.
2. Install the Rust toolchain with `rustfmt` and `clippy`.
3. Cache with `Swatinem/rust-cache@v2`.
4. `cargo fmt --all -- --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo test --workspace`
7. `cargo xtask validate`
8. `cargo xtask e2e`

Timeout: 30 minutes. Upload the E2E temp directory as an artifact on failure.

The CI file lives at `.github/workflows/rustlab-ci.yml` at the repository root, because GitHub only reads workflows from the root `.github/` directory. Set `working-directory: rustlab` on every Cargo step. Do not create two CI files.

## Fixtures

`rustlab/fixtures/hello-cli` is owned here. It is a read-only fixture for the runner and CLI tests:

```text
rustlab/fixtures/hello-cli/
  Cargo.toml
  src/main.rs
  tests/hello.rs
```

The binary prints `hello`. The test asserts it. Do not add external dependencies.

## Tests

- `cargo xtask validate` on the real content: all pass.
- A deliberately broken solution fails validation with a clear message.
- `cargo xtask e2e` passes on Windows.
- `cargo xtask e2e --project unit-converter --milestone parse` runs only that milestone.
- A missing `RUSTLAB_CONTENT` produces a content error, not a panic.
- The E2E leaves no files outside the temp directory.

## Acceptance

From `rustlab/`:

```text
cargo run -p xtask -- validate
cargo run -p xtask -- e2e
cargo clippy -p xtask --all-targets -- -D warnings
cargo fmt -p xtask -- --check
```

CI must be green on all three platforms before the MVP gate.

## Deliverable

A PR titled `e2e: content validation, black-box E2E, and CI matrix`.

The PR description must include the validation table, the E2E transcript, and a link to a green CI run.

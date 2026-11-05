# WS8: Docs and developer experience

Agent: docs
Branch: `ws/docs`
Wave: 2
Depends on: WS4, WS6
Merge order: 9

## Goal

Make the repo usable by a new learner and a new contributor. The quickstart must work from a clean clone. The content guide must be enough to add a second project without reading the source.

## Owns

- `rustlab/README.md`
- `rustlab/CONTRIBUTING.md`
- `rustlab/docs/quickstart.md`
- `rustlab/docs/architecture.md`
- `rustlab/docs/content-authoring.md`
- `rustlab/docs/troubleshooting.md`
- `rustlab/docs/release.md`

## Must not touch

- The root `README.md`, `RUST_NOTES.md`, or the existing `learn_rust` package
- `docs/build-plan/contracts.md`
- `docs/build-plan/status.md`
- `docs/learn-rust-product.md`
- `rustlab/crates/**`
- `rustlab/content/**`

## Depends on

The CLI and E2E behavior from WS4 and WS6. Do not document commands that do not exist yet.

## Deliverables

### README

- One paragraph: what `rustlab` is.
- Install from source.
- The first project, in five commands.
- The command list.
- A short "how it works" section.
- Links to the quickstart, content guide, and troubleshooting.
- Status: MVP, local-first, one project.
- License.

### CONTRIBUTING

- Prerequisites: Rust stable, a linker, Git.
- Setup: clone, `cd rustlab`, `cargo test --workspace`.
- Worktree and branch rules, linking to `../docs/build-plan/coordination.md` from `rustlab/CONTRIBUTING.md`.
- How to run the checks: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo xtask validate`, `cargo xtask e2e`.
- How to add a project, linking to `docs/content-authoring.md`.
- PR expectations.

### docs/quickstart.md

A 10-minute path:

1. Install Rust.
2. Install `rustlab`.
3. Run `rustlab doctor`.
4. Run `rustlab start unit-converter`.
5. Open the project in an editor.
6. Run `rustlab check`.
7. Use `rustlab hint` when stuck.
8. Run `rustlab next`.

Include the expected output at each step. Test it from a clean shell.

### docs/architecture.md

- Crate graph.
- The check lifecycle: content -> plan -> run -> evaluate -> report -> progress.
- The runner boundary.
- The content format.
- Why content is Cargo, not a database.
- Link to the contracts for exact types.

### docs/content-authoring.md

- The directory layout.
- The track and project manifests.
- How to write a milestone, lesson, task, and hints.
- Hidden tests and how they are injected.
- Solutions and broken variants.
- How to run `cargo xtask validate`.
- Common mistakes: absolute paths, duplicate IDs, tests that assert message text.

### docs/troubleshooting.md

- Windows: missing MSVC build tools, `link.exe` not found, antivirus blocking `target/`, long paths.
- `cargo` not found after install: restart the shell, check `PATH`.
- Proxy or offline: `CARGO_NET_OFFLINE`, `HTTP_PROXY`.
- `RUSTLAB_HOME` permission errors.
- A check times out: run `rustlab check --json` and inspect the detail.
- A project marker is missing: run `rustlab start` again or `rustlab doctor`.

### docs/release.md

- Versioning.
- Changelog format.
- Build binaries for Windows, macOS, Linux.
- Checksums.
- Run `cargo xtask validate` and `cargo xtask e2e` on the release commit.
- Smoke test: install from the artifact, run `rustlab doctor`, finish milestone 1.
- Rollback plan.

## Tests

- Run every command in the quickstart from a clean clone. Fix the docs if a command fails.
- Check every relative link with a small script or `lychee` if available. Do not add a new CI dependency without asking the integrator.
- Confirm the README command list matches `rustlab --help`.
- Confirm the content guide matches the real manifest schema. Link to the contracts instead of copying the schema. From `rustlab/docs/`, the contracts are at `../../docs/build-plan/contracts.md`.

## Acceptance

- A new contributor can follow `rustlab/CONTRIBUTING.md` from a clean clone and run `cargo xtask e2e`.
- A new learner can follow `rustlab/docs/quickstart.md` and finish milestone 1.
- No broken relative links.
- No duplicated schema that can drift from `docs/build-plan/contracts.md`.

## Deliverable

A PR titled `docs: quickstart, architecture, content guide, and troubleshooting`.

The PR description must include the terminal transcript of the quickstart and a list of links checked.

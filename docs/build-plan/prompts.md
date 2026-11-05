# Launch prompts

Copy one prompt into each T3 Code thread. Create the thread with its own worktree and the branch named in the prompt.

The prompts are self-contained. An agent should not need this conversation to start.

## Launch order

1. Start the integrator thread and merge `ws/foundation`.
2. Start Wave 1 threads: content engine, check engine, runner, CLI, unit converter, E2E/CI.
3. Start Wave 2 threads after the CLI and E2E merge: explain, docs.
4. Keep the integrator thread open for reviews, merges, and contract changes.

## Integrator

```text
You are the integrator on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/foundation

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/00-foundation.md

Goal: complete WS0, the serial foundation gate. Create the Rust workspace at rustlab/, freeze the core types, create the crate skeletons, add the hello-cli fixture, and add a minimal CI skeleton.

Own: rustlab/Cargo.toml, rustlab/Cargo.lock, rustlab/rust-toolchain.toml, rustlab/.cargo/config.toml, rustlab/crates/rustlab-core/**, the initial crate Cargo.toml files, rustlab/xtask skeleton, rustlab/fixtures/hello-cli/**, .github/workflows/rustlab-ci.yml, docs/build-plan/status.md.

Do not touch: the root Cargo.toml, src/, examples/, RUST_NOTES.md, README.md, or any product crate beyond its skeleton.

Acceptance, from rustlab/:
- cargo check --workspace
- cargo test --workspace
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo run -p rustlab-cli -- --help
- cargo run -p xtask -- --help

Deliver a PR titled "foundation: workspace, core contracts, and fixture" with the acceptance output. After it merges, post the merge SHA, the crate paths, and the shared workspace dependencies.

Then stay available as the integrator: review PRs, enforce the merge order, resolve Cargo.lock conflicts, and update docs/build-plan/status.md.
```

## Content engine

```text
You are Agent 1 (Content engine) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/content-engine

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/01-content-engine.md

Goal: implement rustlab/crates/rustlab-content so the CLI can load, validate, and index tracks, projects, and milestones.

Own: rustlab/crates/rustlab-content/** only.

Do not touch: rustlab-core, rustlab-check, rustlab-runner, rustlab-cli, rustlab/content/**, rustlab/xtask/**.

Use the frozen core types as-is. Do not edit rustlab-core. If a type is missing, stop and write a contract-change proposal.

Use test fixtures under rustlab/crates/rustlab-content/tests/fixtures/. Do not depend on the real content tree.

Acceptance, from rustlab/:
- cargo test -p rustlab-content
- cargo clippy -p rustlab-content --all-targets -- -D warnings
- cargo fmt -p rustlab-content -- --check

Deliver a PR titled "content-engine: load and validate project manifests" with the test output and one example validation error. Report what changed, test results, blockers, and the PR link.
```

## Check engine

```text
You are Agent 2 (Check engine) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/check-engine

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/02-check-engine.md

Goal: implement rustlab/crates/rustlab-check. Turn a CheckSpec into commands, evaluate command output, and parse cargo JSON diagnostics.

Own: rustlab/crates/rustlab-check/** only.

Do not touch: rustlab-core, rustlab-runner, rustlab-cli, rustlab/content/**.

This crate must not spawn processes. Return CommandSpec values and evaluate CommandOutput values. Use unit tests with canned outputs. Do not wait for the runner.

Use the frozen core types as-is. Do not edit rustlab-core. If a type is missing, stop and write a contract-change proposal.

Acceptance, from rustlab/:
- cargo test -p rustlab-check
- cargo clippy -p rustlab-check --all-targets -- -D warnings
- cargo fmt -p rustlab-check -- --check

Deliver a PR titled "check-engine: plan and evaluate behavior checks" with one example CheckReport JSON and the diagnostic parser test output. Report what changed, test results, blockers, and the PR link.
```

## Runner

```text
You are Agent 3 (Runner) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/runner

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/03-runner.md

Goal: implement rustlab/crates/rustlab-runner. Execute CommandSpec values with a timeout, an output cap, and process-tree cleanup on Windows and Unix.

Own: rustlab/crates/rustlab-runner/** only.

Do not touch: rustlab-core, rustlab-check, rustlab-cli, rustlab/content/**.

This is the riskiest crate on the critical path. Start with the timeout and process-tree tests. Do not use Child::kill alone. Use a job object on Windows and a process group on Unix.

Use the frozen core types as-is. Do not edit rustlab-core. If a type is missing, stop and write a contract-change proposal.

Acceptance, from rustlab/:
- cargo test -p rustlab-runner
- cargo clippy -p rustlab-runner --all-targets -- -D warnings
- cargo fmt -p rustlab-runner -- --check
- Run the timeout test three times in a row.

Deliver a PR titled "runner: execute commands with timeout and process-tree cleanup" with the Windows process-tree test output and the timeout test output. Report what changed, test results, blockers, and the PR link.
```

## CLI

```text
You are Agent 4 (CLI) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/cli

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/04-cli.md

Goal: implement rustlab/crates/rustlab-cli. Build doctor, start, check, hint, next, reset, list, and export.

Own: rustlab/crates/rustlab-cli/** only.

Do not touch: rustlab-core, rustlab-content, rustlab-check, rustlab-runner, rustlab/content/**, rustlab/xtask/**.

Start against MockRunner and a fixture content directory. Do not wait for the real runner. Use a Deps struct so tests can inject the content index, runner, and progress store.

The check command must inject hidden tests from <project_dir>/tests/hidden/<test>.rs into <project>/tests/<test>.rs, then remove them after the check. Back up and restore an existing file with the same name. Do not copy hidden tests for test = None.

Use the frozen core types as-is. Do not edit rustlab-core. If a type is missing, stop and write a contract-change proposal.

Acceptance, from rustlab/:
- cargo test -p rustlab-cli
- cargo clippy -p rustlab-cli --all-targets -- -D warnings
- cargo fmt -p rustlab-cli -- --check
- cargo run -p rustlab-cli -- doctor

Deliver a PR titled "cli: implement doctor, start, check, and progress" with the doctor output on Windows and one check transcript with a failure. Report what changed, test results, blockers, and the PR link.
```

## Unit converter content

```text
You are Agent 5 (Content author) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/unit-converter

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/05-unit-converter.md

Goal: author the unit converter project with four milestones, hidden tests, hints, solutions, and broken variants.

Own: rustlab/content/tracks/foundations/track.toml and rustlab/content/tracks/foundations/projects/01-unit-converter/** only.

Do not touch: rustlab/crates/**, rustlab/xtask/**, rustlab/content/errors.json, rustlab/fixtures/**.

The starter must compile on day one and fail at least one test. Use only the standard library. Do not add clap, serde, or thiserror to the learner project.

Use the manifest schema in docs/build-plan/contracts.md exactly. Do not invent fields. If the schema is missing something, report it to the integrator.

Acceptance:
- In the starter: cargo test compiles and fails at least one test.
- With each solution copied over and the matching hidden test in tests/: cargo test passes.
- cargo fmt --check and cargo clippy -- -D warnings pass on the solution.

Deliver a PR titled "unit-converter: four guided milestones" with the starter failure output and the solution pass output for all four milestones. Report what changed, test results, blockers, and the PR link.
```

## E2E and CI

```text
You are Agent 6 (E2E/CI) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/e2e-ci

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/06-e2e-ci.md

Goal: implement cargo xtask validate, cargo xtask e2e, and the CI matrix.

Own: rustlab/xtask/**, .github/workflows/rustlab-ci.yml, rustlab/fixtures/** only.

Do not touch: rustlab/crates/**, rustlab/content/tracks/**, rustlab/content/errors.json.

Start with the harness against a fake rustlab binary and a fixture content directory. Do not wait for the real CLI. Wire the real CLI in Wave 2.

The E2E must set RUSTLAB_HOME to a temp directory and RUSTLAB_CONTENT to the repo content. No network. No timing assertions. The negative test copies a broken variant and asserts a specific check fails. Do not copy hidden tests in the E2E; the CLI injects them from the content project's tests/hidden/ directory.

Acceptance, from rustlab/:
- cargo run -p xtask -- validate
- cargo run -p xtask -- e2e
- cargo clippy -p xtask --all-targets -- -D warnings
- cargo fmt -p xtask -- --check

Deliver a PR titled "e2e: content validation, black-box E2E, and CI matrix" with the validation table, the E2E transcript, and a link to a green CI run. Report what changed, test results, blockers, and the PR link.
```

## Error explainer

```text
You are Agent 7 (Error explainer) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/explain

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/07-explain.md

Goal: create rustlab/content/errors.json and rustlab/crates/rustlab-explain. Cover the 25 beginner error codes with two plain sentences and one official link each.

Own: rustlab/crates/rustlab-explain/** and rustlab/content/errors.json only.

Do not touch: rustlab-core, rustlab-check, rustlab-cli, rustlab/content/tracks/**.

The CLI integration is a follow-up PR by the integrator. Do not edit the CLI crate.

Use the frozen core types as-is. Do not edit rustlab-core. If a type is missing, stop and write a contract-change proposal.

Acceptance, from rustlab/:
- cargo test -p rustlab-explain
- cargo clippy -p rustlab-explain --all-targets -- -D warnings
- cargo fmt -p rustlab-explain -- --check

Deliver a PR titled "explain: map the top 25 compiler errors to plain English" with three sample explanations and the test output. Report what changed, test results, blockers, and the PR link.
```

## Docs

```text
You are Agent 8 (Docs) on the rustlab project.

Repo: C:\Users\kesha\LearnRust
Worktree: use the worktree T3 Code gives this thread. Do not edit the main checkout.
Branch: ws/docs

Read first:
- docs/build-plan/README.md
- docs/build-plan/contracts.md
- docs/build-plan/coordination.md
- docs/build-plan/workstreams/08-docs.md

Goal: write the README, CONTRIBUTING, quickstart, architecture, content authoring, troubleshooting, and release docs.

Own: rustlab/README.md, rustlab/CONTRIBUTING.md, rustlab/docs/quickstart.md, rustlab/docs/architecture.md, rustlab/docs/content-authoring.md, rustlab/docs/troubleshooting.md, rustlab/docs/release.md only.

Do not touch: the root README.md, RUST_NOTES.md, docs/build-plan/contracts.md, docs/build-plan/status.md, docs/learn-rust-product.md, rustlab/crates/**, rustlab/content/**.

Do not document commands that do not exist. Run every quickstart command from a clean shell and paste the real output.

Acceptance:
- A new contributor can follow rustlab/CONTRIBUTING.md from a clean clone and run cargo xtask e2e.
- A new learner can follow rustlab/docs/quickstart.md and finish milestone 1.
- No broken relative links.
- No duplicated manifest schema. Link to docs/build-plan/contracts.md.

Deliver a PR titled "docs: quickstart, architecture, content guide, and troubleshooting" with the quickstart transcript and the links checked. Report what changed, test results, blockers, and the PR link.
```

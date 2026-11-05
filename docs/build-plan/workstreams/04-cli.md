# WS4: CLI

Agent: CLI
Branch: `ws/cli`
Wave: 1, start against mocks; integrate in Wave 2
Depends on: WS1, WS2, WS3
Merge order: 6

## Goal

Build the learner-facing `rustlab` binary. It orchestrates content, checks, the runner, and progress. It must be usable before the real runner lands, so start with `MockRunner` and a fixture content directory.

## Owns

- `rustlab/crates/rustlab-cli/**`

## Must not touch

- `rustlab/crates/rustlab-core/**`
- `rustlab/crates/rustlab-content/**`
- `rustlab/crates/rustlab-check/**`
- `rustlab/crates/rustlab-runner/**`
- `rustlab/content/**`
- `rustlab/xtask/**`

## Depends on

- Content index from WS1.
- Check planning and evaluation from WS2.
- `ProcessRunner` from WS3.
- Frozen core types from WS0.

## Commands

```text
rustlab doctor [--json]
rustlab list [--json]
rustlab start [<project>] [--dir <path>] [--force] [--json]
rustlab check [--json] [--project <id>] [--milestone <id>]
rustlab hint [--level <1|2|3>]
rustlab next [--json]
rustlab reset [--yes]
rustlab export [--output <path>]
```

Exit codes: 0 success, 1 check failed, 2 usage, 3 environment, 4 content.

## Behavior

### doctor

Check, in order:

1. `rustc --version` runs and reports a version.
2. `cargo --version` runs.
3. A temporary hello-world program compiles and runs with `rustc`. This catches a missing linker, which is the most common Windows setup failure.
4. The content index loads.
5. `RUSTLAB_HOME` exists or can be created and written.

Print a table of check names, status, and detail. `--json` prints the object from the contracts. Exit 3 if any check fails.

### start

1. Load the content index.
2. Pick the project from the argument, or the first incomplete project.
3. Pick the first incomplete milestone.
4. Copy the starter directory to `--dir`, defaulting to `./<project-id>`.
5. Refuse to overwrite an existing directory unless `--force`.
6. Write `<project>/.rustlab/project.json`.
7. Set the current milestone in progress.
8. Print the milestone task and the command to run next.

### check

1. Resolve the project and milestone from `--project`/`--milestone`, the marker, or the current directory.
2. For each check: call `rustlab-check::plan`, run each command with the runner, then `rustlab-check::evaluate`.
3. Parse diagnostics from the build and test outputs.
4. Print a summary: one line per check, then the first failing detail.
5. If all checks pass, mark the milestone complete and set the next milestone as current.
6. If any check fails, exit 1.

Run checks sequentially for deterministic output. Parallel checks are a later optimization.

### Hidden test injection

For each `cargo_test` check with `test = Some(name)`:

1. Find `<project_dir>/tests/hidden/<name>.rs`.
2. Copy it to `<project>/tests/<name>.rs`.
3. If the file already exists, back it up and restore it after the check.
4. Remove the injected file after the check, even on failure.

The learner must never edit the hidden test. Do not copy hidden tests for `test = None`. If a hidden test file is missing, exit 4 with a content error before running any checks.

The content validator does the same copy in its temp directory. The E2E relies on the CLI injection and does not copy hidden tests itself.

### hint

Print the next hint for the current milestone. `--level` selects a specific hint. If no hint exists, say so and exit 0.

### next

If the current milestone is complete, advance and print the next task. If not, print "run rustlab check first" and exit 1. If the project is complete, print the completion message and suggest the next project.

### reset

Copy the project starter over the current project after a confirmation prompt. Preserve `.rustlab/`. With `--yes`, skip the prompt. This is a destructive command; say exactly which files will be overwritten.

### list

Show tracks, projects, and milestones with completion counts. `--json` prints the object from the contracts.

### export

Print the progress JSON to stdout, or write it to `--output`.

## Content discovery

Resolve the content root in this order:

1. `RUSTLAB_CONTENT`.
2. `./content` if it contains `tracks/`.
3. `<directory of the rustlab binary>/content`.
4. `<directory of the rustlab binary>/../content`.

If none exists, exit 4 with a message that names the paths checked. The release package ships `content/` next to the binary.

## Progress store

- Path: `RUSTLAB_HOME/state.json`.
- `RUSTLAB_HOME` defaults to the platform data directory.
- Atomic write: write `state.json.tmp`, then rename.
- A corrupt file is moved to `state.json.bak` and replaced with a fresh state, with a warning.
- `learner_id` is a UUID v4 created on first run.

Keep the store in `rustlab-cli/src/progress.rs` for the MVP. The web app can extract it later.

## Output

- Human output uses `✓`, `✗`, and `!` for status. No emojis.
- `--json` prints only JSON to stdout. Human messages go to stderr.
- Respect `NO_COLOR`.
- Keep the first failure visible without scrolling.

Example:

```text
$ rustlab check
✓ parses "10 km" as 10.0
✓ converts km to miles
✗ rejects an unknown unit
    expected: error "unknown unit: furlong"
    got:      panic at src/main.rs:42
    hint: return Err(...) instead of unwrap()

1 of 3 checks failed. Run `rustlab hint` for a nudge.
```

## Testability

Define a `Deps` struct:

```rust
pub struct Deps {
    pub content: Box<dyn ContentProvider>,
    pub runner: Box<dyn ProcessRunner>,
    pub progress: ProgressStore,
}
```

`main` builds the real dependencies. Tests build `Deps` with a `FakeContent` and a test-local `MockRunner`. This avoids a hidden test-only environment variable and lets the CLI work before WS1 and WS3 land.

## Tests

- `doctor --json` on the CI machine: rustc, cargo, and hello-world pass.
- `list --json` with a fixture content directory.
- `start` into a temp directory: files copied, marker written, progress current set.
- `start` refuses an existing directory without `--force`.
- `check` with `MockRunner` passing outputs: report passed, progress updated.
- `check` with a failing output: exit 1, progress unchanged.
- `hint` prints the first and second hint.
- `next` refuses before completion and advances after.
- `reset` restores a modified file and preserves `.rustlab/`.
- `export` round-trips the progress JSON.
- Corrupt `state.json` is backed up and replaced.

Use `assert_cmd` for the binary-level tests and direct command functions for the mock-runner tests.

## Acceptance

From `rustlab/`:

```text
cargo test -p rustlab-cli
cargo clippy -p rustlab-cli --all-targets -- -D warnings
cargo fmt -p rustlab-cli -- --check
cargo run -p rustlab-cli -- doctor
```

## Deliverable

A PR titled `cli: implement doctor, start, check, and progress`.

The PR description must include the `doctor` output on Windows and one `check` transcript with a failure.

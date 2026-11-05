# WS3: Runner

Agent: runner
Branch: `ws/runner`
Wave: 1
Depends on: WS0
Merge order: 4

## Goal

Execute a `CommandSpec` reliably, with a timeout, an output cap, and no orphaned child processes. This is the riskiest crate on the critical path. Start it first in Wave 1.

## Owns

- `rustlab/crates/rustlab-runner/**`

## Must not touch

- `rustlab/crates/rustlab-core/**`
- `rustlab/crates/rustlab-check/**`
- `rustlab/crates/rustlab-cli/**`
- `rustlab/content/**`

## Depends on

The frozen `CommandSpec`, `CommandOutput`, and `ProcessRunner` types in `rustlab-core`.

## Deliverables

- `LocalRunner` implementing `ProcessRunner`.
- `MockRunner` implementing `ProcessRunner` with a queue of canned outputs.
- Process-tree cleanup on Windows and Unix.
- Timeout enforcement.
- Output cap enforcement.
- Environment handling.

## Behavior

`LocalRunner::run` must:

1. Spawn the program with `args`, `cwd`, `stdin`, and the merged environment.
2. Capture stdout and stderr on separate threads so a full pipe cannot deadlock.
3. Enforce `timeout_secs`. On timeout, kill the process tree, then wait.
4. Enforce `output_cap_bytes` across stdout and stderr. On overflow, kill the process tree and return an error.
5. Return `CommandOutput` with `status`, `stdout`, `stderr`, `timed_out`, and `duration_ms`.
6. Never inherit `RUSTLAB_*` variables except the ones explicitly passed in `cmd.env`.

Environment merge: start from the parent environment, remove all `RUSTLAB_*`, then apply `cmd.env`. Keep `PATH`, `HOME`, `USERPROFILE`, `CARGO_HOME`, `RUSTUP_HOME`, `TEMP`, and `TMP`.

## Process-tree cleanup

Windows:

- Create a job object with `CreateJobObjectW`.
- Set `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`.
- Assign the child with `AssignProcessToJobObject`.
- On timeout or output cap, call `TerminateJobObject`.
- Use `windows-sys` with the needed features.

Unix:

- Set `process_group(0)` on the child.
- On timeout or output cap, send `SIGKILL` to `-pgid`.
- Use `libc` or `rustix`.

Do not use `Child::kill` alone. It kills one process, not the tree.

## MockRunner

```rust
pub struct MockRunner { /* ... */ }

impl MockRunner {
    pub fn new(outputs: Vec<CommandOutput>) -> Self;
    pub fn push(&self, output: CommandOutput);
}
```

`MockRunner::run` pops the next output. If the queue is empty, return an error that names the command. It records the commands it received so tests can assert them.

## Tests

- Echo: run a shell command that prints `hello`, assert stdout.
- Exit code: run a command that exits 3, assert status.
- Timeout: run a sleep command with a 1s timeout, assert `timed_out` and elapsed under 3s.
- Output cap: run a command that prints more than the cap, assert an error.
- Environment: pass `RUSTLAB_TEST=1`, assert the child sees it.
- Environment scrub: set `RUSTLAB_SECRET=1` in the parent, assert the child does not see it unless passed.
- MockRunner: queue order, empty queue error, recorded commands.
- Windows-only: spawn `cmd /C ping -t 127.0.0.1`, time out, assert the process is gone. Use `tasklist` or a PID check.
- Unix-only: spawn `sh -c 'sleep 30 & wait'`, time out, assert the process group is gone.

Use `cfg(windows)` and `cfg(unix)` for the platform-specific tests. The cross-platform tests must pass on all three CI runners.

## Acceptance

From `rustlab/`:

```text
cargo test -p rustlab-runner
cargo clippy -p rustlab-runner --all-targets -- -D warnings
cargo fmt -p rustlab-runner -- --check
```

Also run the timeout test three times in a row to catch flakiness:

```text
cargo test -p rustlab-runner timeout -- --nocapture
cargo test -p rustlab-runner timeout -- --nocapture
cargo test -p rustlab-runner timeout -- --nocapture
```

## Deliverable

A PR titled `runner: execute commands with timeout and process-tree cleanup`.

The PR description must include the Windows process-tree test output and the timeout test output.

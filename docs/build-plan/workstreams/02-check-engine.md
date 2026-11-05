# WS2: Check engine

Agent: check
Branch: `ws/check-engine`
Wave: 1
Depends on: WS0
Merge order: 3

## Goal

Turn a `CheckSpec` into deterministic commands, and turn command output into a pass or fail with a useful message. This crate does not spawn processes. The runner does that.

## Owns

- `rustlab/crates/rustlab-check/**`
- `rustlab/crates/rustlab-check/tests/fixtures/**`

## Must not touch

- `rustlab/crates/rustlab-core/**`
- `rustlab/crates/rustlab-runner/**`
- `rustlab/crates/rustlab-cli/**`
- `rustlab/content/**`

## Depends on

The frozen `CheckSpec`, `CommandSpec`, `CommandOutput`, `CheckResult`, and `Diagnostic` types in `rustlab-core`.

## Deliverables

```rust
pub fn plan(spec: &CheckSpec, ctx: &CheckContext) -> anyhow::Result<Vec<CommandSpec>>;
pub fn evaluate(spec: &CheckSpec, outputs: &[CommandOutput]) -> CheckResult;
pub fn parse_cargo_diagnostics(stdout: &str, stderr: &str) -> Vec<Diagnostic>;
```

## Check kinds

### `cargo_test`

Plan:

```text
cargo test --package <package> [--test <test>]
```

`test` is the integration test target name, not a function name. `None` runs all tests in the package. Evaluate: exit code 0 and not timed out.

### `cli`

Plan:

```text
cargo build --package <package>
<target-dir>/debug/<binary> <args...>
```

The first command builds the binary. The second runs it with `args`, `stdin`, and the check timeout. The binary path is `ctx.binary_path` when set, otherwise `ctx.target_dir/debug/<package>` with `.exe` on Windows.

Evaluate the second command's exit code, stdout, and stderr against `expect`. The build command's failure is reported as a failed check with the build output in `detail`.

### `compile_fail`

Plan:

```text
cargo check --package <package> --message-format=json
```

Evaluate: parse diagnostics, assert at least one error with `code == error_code`. If the package compiles, fail with "expected error <code>, but the package compiled".

This is used by content validation and negative E2E tests, not by normal learner milestones.

### `clippy`

Plan:

```text
cargo clippy --package <package> -- -D <lint>...
```

Evaluate: exit code 0. If `deny` is empty, use `-D warnings`.

## Matchers

Apply to trimmed output.

| Matcher | Behavior |
| --- | --- |
| `Empty` | Output must be empty after trim. |
| `Exact { value }` | Output must equal `value` after trim. |
| `Contains { value }` | Output must contain `value`. |
| `Regex { value }` | Output must match the regex. |
| `Json { value }` | Output must parse as JSON and equal `value`. |

`Expect.exit_code` defaults to 0. A timeout always fails.

## Diagnostics

Parse `cargo check --message-format=json` output from both stdout and stderr. Each line may be a JSON object with `reason: "compiler-message"`.

Map:

- `message.level` to `DiagnosticLevel`.
- `message.code.code` to `Diagnostic.code`.
- `message.message` to `Diagnostic.message`.
- `message.spans` to `Span`.
- `message.children` recursively.
- `message.children[].spans[].suggested_replacement` to `Diagnostic.suggestions`.

Ignore non-JSON lines. Deduplicate by `(code, message, primary span)`. Keep at most 20 diagnostics, errors first.

## Implementation notes

- Do not spawn processes. Do not read the filesystem except for `cargo metadata`-style fixtures in tests.
- Use `ctx.target_dir` from the caller. Do not guess `target/`.
- For `cli`, if the package has more than one binary, return an error that names the package. The content engine validates the single-binary rule.
- Truncate `detail` to 8 KB.
- Keep `message` short: one sentence the learner can read.
- Tests use `CommandOutput` values directly. No real Cargo runs in unit tests.
- Add one integration test that runs against `rustlab/fixtures/hello-cli` only if the runner is available. Gate it behind `#[ignore]` so WS2 is not blocked by WS3.

## Tests

- `plan` for each check kind produces the expected program and args.
- `cargo_test` passes on exit 0 and fails on exit 1.
- `cli` passes with matching stdout, fails on wrong exit code, wrong stdout, and timeout.
- `compile_fail` passes when the error code appears, fails when it does not.
- Each matcher, including an invalid regex.
- Diagnostic parser with a captured JSON sample in `tests/fixtures/cargo_check.json`.
- Diagnostic parser ignores non-JSON lines and deduplicates.

## Acceptance

From `rustlab/`:

```text
cargo test -p rustlab-check
cargo clippy -p rustlab-check --all-targets -- -D warnings
cargo fmt -p rustlab-check -- --check
```

## Deliverable

A PR titled `check-engine: plan and evaluate behavior checks`.

The PR description must include one example `CheckReport` JSON and the diagnostic parser test output.

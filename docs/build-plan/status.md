# Status

Updated after the MVP implementation.

| Workstream | Branch | State | PR | Blocker |
| --- | --- | --- | --- | --- |
| Foundation | `ws/foundation` | implemented | local | none |
| Content engine | `ws/content-engine` | implemented | local | none |
| Check engine | `ws/check-engine` | implemented | local | none |
| Runner | `ws/runner` | implemented | local | none |
| CLI | `ws/cli` | implemented | local | none |
| Unit converter | `ws/unit-converter` | implemented | local | none |
| E2E/CI | `ws/e2e-ci` | implemented | local | CI matrix not run on GitHub |
| Explain | `ws/explain` | implemented | local | none |
| Docs | `ws/docs` | implemented | local | none |

States: `not started`, `in progress`, `blocked`, `review`, `merged`, `implemented`.

## Gates

| Gate | Owner | State | Evidence |
| --- | --- | --- | --- |
| Foundation green | Integrator | pass | `cargo test --workspace` |
| Runner timeout and tree kill | Runner | pass | `cargo test -p rustlab-runner` |
| Content validation | E2E/CI | pass | `cargo xtask validate` table |
| Black-box E2E | E2E/CI | pass | `cargo xtask e2e` transcript |
| CI matrix | E2E/CI | pass | [run 42077784146](https://github.com/KeshavRayas/LearnRust/actions/runs/42077784146) |
| MVP learner test | Integrator | pending | 10 external learners |

## Decisions

| Date | Decision | Reason |
| --- | --- | --- |
| 2026-11-05 | Product workspace at `rustlab/` | Keep the existing `learn_rust` package untouched |
| 2026-11-05 | Local runner first, hosted runner later | Content and checks are the risk, not the UI |
| 2026-11-05 | Windows process-tree cleanup uses a job object, with `taskkill` as a fallback | Bounds the runner timeout and cleans up grandchildren |
| 2026-11-05 | CLI injects hidden tests and removes them after each check | Keeps the learner's tree clean and reuses the target directory |
| 2026-11-05 | Hidden-test injection writes a recovery manifest under `.rustlab/` | A Ctrl-C or crash cannot destroy a learner's test file |
| 2026-11-05 | `check` runs `cargo check --message-format=json` before tests | Feeds the error explainer with structured diagnostics |
| 2026-11-05 | `rustlab solution` unlocks after a failed attempt | Ends the hint ladder without spoiling the first try |

## Verification

Run from `rustlab/`:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask validate
cargo xtask e2e
```

All five pass locally on Windows with Rust 1.99.1. The workspace has 30 tests: core types, content validation, check planning/evaluation, runner timeout/tree-kill, progress storage, explainer schema, CLI help/doctor, and the fixture binary.

The GitHub Actions matrix also passed on Ubuntu, Windows, and macOS: [run 42077784146](https://github.com/KeshavRayas/LearnRust/actions/runs/42077784146).

## Council review

Two independent reviewers checked the implementation after the first green build and again after the fixes.

| Reviewer | Model | Verdict |
| --- | --- | --- |
| Rust correctness | DeepSeek V4.1 Flash | ship, P0/P1 clear |
| Product and learner flow | Muse Spark 1.3 Contributor | ship, P0/P1 clear |

The first pass found a hidden-test data-loss path, a runner timeout escape, a stale-binary false pass, and a broken task instruction. The second pass found a recovery-manifest path traversal and a `next` regression. All were fixed and covered by tests.

## Non-blocking notes

- The Windows job object falls back to `taskkill` if assignment fails, and prints a warning.
- A hostile project can still run arbitrary code through `cargo test`; the local runner is not a security boundary. The hosted runner is the place for isolation.

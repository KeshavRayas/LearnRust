# Architecture

The product has four layers.

```text
Surfaces: CLI
    |
Core: domain, progress, checks, hints
    |
Runners: local cargo
    |
Content: tracks, projects, milestones, tests
```

## Crates

| Crate | Responsibility |
| --- | --- |
| `rustlab-core` | Domain types, `ContentProvider`, `ProcessRunner`, `CheckReport` |
| `rustlab-content` | Manifest parsing, path validation, content index |
| `rustlab-check` | Check planning, output evaluation, compiler diagnostics |
| `rustlab-runner` | Local process execution, timeout, output cap, process-tree cleanup |
| `rustlab-explain` | Plain-English compiler error map |
| `rustlab-cli` | Commands, progress store, human and JSON output |
| `xtask` | Content validation and black-box E2E |

## Check lifecycle

1. The CLI loads the content index and the project marker.
2. It injects hidden tests into the learner's `tests/` directory.
3. `rustlab-check::plan` turns each `CheckSpec` into one or more `CommandSpec` values.
4. `rustlab-runner` runs the commands and captures output.
5. `rustlab-check::evaluate` turns the output into a `CheckResult`.
6. The CLI builds a `CheckReport`, prints it, and updates progress.
7. The injected tests are removed.

## Content

Content is a normal directory tree:

```text
content/tracks/<track>/track.toml
content/tracks/<track>/projects/<dir>/project.toml
```

Each project has a starter Cargo package, lessons, tasks, hints, hidden tests, solutions, and broken variants. The manifest schema is in [../../docs/build-plan/contracts.md](../../docs/build-plan/contracts.md).

## Why local first

The learner needs the real toolchain, real crates, and a real editor. The local runner avoids a sandbox, works offline, and keeps checks fast. The `ProcessRunner` trait leaves room for a hosted container runner later.

## Progress

Progress is a JSON file at `RUSTLAB_HOME/state.json`. Writes are atomic. A corrupt file is backed up and replaced.

## Failure behavior

A failed check prints the failing check id, a short message, and a detail excerpt. If the compiler produced a known error code, the explainer prints two sentences and a link.

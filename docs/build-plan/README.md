# Option B build plan: parallel agent workstreams

Goal: ship the local-first `rustlab` MVP with multiple agents working in separate T3 Code threads and worktrees.

This plan assumes the product direction in [../learn-rust-product.md](../learn-rust-product.md): a Rust CLI that scaffolds real Cargo projects, checks behavior, explains failures, and stores progress locally.

## Workspace root

The product is a new Cargo workspace at `rustlab/` inside this repository. The existing `learn_rust` package at the repo root stays untouched.

All paths below are relative to the repo root unless they start with `rustlab/`. Run Cargo commands from `rustlab/`.

## MVP scope

The first release has:

- `rustlab doctor`, `start`, `check`, `hint`, `next`, `reset`, `list`, `export`
- One guided project: the unit converter, with four milestones
- A local Cargo runner with timeouts and process-tree cleanup
- Behavior checks, hidden tests, and progress in `RUSTLAB_HOME/state.json`
- `cargo xtask validate` and `cargo xtask e2e`
- CI on Windows, macOS, and Linux

Not in the first release: web UI, accounts, hosted sandbox, AI tutor, TUI dashboard, more projects.

## Why this shape

One serial gate freezes the interfaces. Then six workstreams run in parallel with disjoint file ownership. The CLI is the integration point. The E2E harness is the release gate.

The critical path is:

```text
Foundation -> Runner -> CLI -> E2E -> release
```

The runner is the riskiest piece. Start it first in Wave 1.

## Waves

| Wave | Workstreams | Exit criteria |
| --- | --- | --- |
| 0 | Foundation | `cargo check --workspace` and `cargo test --workspace` pass. Contracts are merged. |
| 1 | Content engine, Check engine, Runner, CLI (against mocks), Unit converter, E2E harness | Every PR is green. No contract changes. |
| 2 | CLI integration, E2E integration, Error explainer, Docs | `cargo xtask e2e` passes on Windows. |
| 3 | Hardening, cross-platform, release | CI matrix green. Install from path works. 10 external learners finish milestone 1. |

## Dependency graph

```text
                         WS0 Foundation
                                |
        +-----------+-----------+-----------+-----------+
        |           |           |           |           |
   WS1 Content  WS2 Check   WS3 Runner  WS5 Content  WS6 E2E
        |           |           |        (unit conv)  (harness)
        +-----------+-----+-----+-----------+-----------+
                          |
                       WS4 CLI
                          |
              +-----------+-----------+
              |                       |
         WS7 Explain              WS6 E2E (wire)
              |                       |
              +-----------+-----------+
                          |
                       WS8 Docs
```

## Workstreams

| WS | Agent | Branch | Owns | Depends on | Merge order |
| --- | --- | --- | --- | --- | --- |
| 0 | Integrator | `ws/foundation` | Workspace, core types, CI skeleton, fixture | none | 1 |
| 1 | Content | `ws/content-engine` | `rustlab/crates/rustlab-content/**` | 0 | 2 |
| 2 | Check | `ws/check-engine` | `rustlab/crates/rustlab-check/**` | 0 | 3 |
| 3 | Runner | `ws/runner` | `rustlab/crates/rustlab-runner/**` | 0 | 4 |
| 4 | CLI | `ws/cli` | `rustlab/crates/rustlab-cli/**` | 1, 2, 3 | 6 |
| 5 | Content author | `ws/unit-converter` | `rustlab/content/tracks/foundations/track.toml`, `rustlab/content/tracks/foundations/projects/01-unit-converter/**` | 0 | 5 |
| 6 | E2E/CI | `ws/e2e-ci` | `rustlab/xtask/**`, `.github/workflows/rustlab-ci.yml`, `rustlab/fixtures/**` | 0, 4, 5 | 7 |
| 7 | Explain | `ws/explain` | `rustlab/crates/rustlab-explain/**`, `rustlab/content/errors.json` | 0, 2, 4 | 8 |
| 8 | Docs | `ws/docs` | `rustlab/README.md`, `rustlab/CONTRIBUTING.md`, `rustlab/docs/**` | 4, 6 | 9 |

## Merge order

1. `ws/foundation`
2. `ws/content-engine`
3. `ws/check-engine`
4. `ws/runner`
5. `ws/unit-converter`
6. `ws/cli`
7. `ws/e2e-ci`
8. `ws/explain`
9. `ws/docs`

The order is chosen so no two PRs touch the same files. The CLI merges after its dependencies. E2E merges after the CLI. Docs merge last.

## Files to read first

Every agent reads:

- [contracts.md](contracts.md)
- [coordination.md](coordination.md)
- The workstream brief for its own thread
- [prompts.md](prompts.md) for the launch prompt

The integrator also owns [status.md](status.md).

## Definition of done for the MVP

- From the repo root, `cargo install --path rustlab/crates/rustlab-cli` succeeds, and `RUSTLAB_CONTENT=rustlab/content rustlab doctor` passes.
- `rustlab doctor` passes on Windows with Rust and the MSVC linker.
- `rustlab start unit-converter` creates a project and prints the first task.
- `rustlab check` runs the milestone checks and finishes in under five seconds on an incremental build.
- `rustlab hint` prints the next hint.
- `rustlab next` advances only after all checks pass.
- The unit converter has four milestones. The starter fails at least one check. The solution passes all of them.
- `cargo xtask validate` passes.
- `cargo xtask e2e` passes on Windows, macOS, and Linux.
- A new learner can finish milestone 1 in under 15 minutes.

## Risks

**Contract drift.** Agents edit `rustlab-core` to make their crate compile. Mitigation: core is frozen after WS0. Changes go through a contract-change PR approved by the integrator.

**Cargo.lock conflicts.** Every dependency addition touches the workspace lockfile. Mitigation: the integrator owns `Cargo.lock` and resolves conflicts. Agents do not hand-edit it.

**Windows process-tree cleanup.** `cargo test` can leave child processes behind. Mitigation: the runner uses a job object on Windows and a process group on Unix. Test this in WS3, not at release.

**E2E flakiness.** Network, timing, and global state make E2E unreliable. Mitigation: `RUSTLAB_HOME` and `RUSTLAB_CONTENT` isolate every run. No network. No timing assertions.

**Content and test coupling.** Hidden tests that assert exact error strings break on compiler upgrades. Mitigation: assert error codes and parsed values, not message text.

**Agent overlap.** Two agents edit the same file and create merge conflicts. Mitigation: the ownership table in [coordination.md](coordination.md) is enforced. If a file is not listed as owned, do not edit it.

**Scope creep.** Web, accounts, and AI consume the schedule. Mitigation: they are explicitly out of scope until the MVP gate passes.

## How to launch

1. Merge this plan and WS0 in one thread.
2. Create Wave 1 threads with their own worktrees. Paste the matching prompt from [prompts.md](prompts.md).
3. The integrator reviews and merges in the order above.
4. Start Wave 2 after the CLI and E2E are integrated.
5. Run the MVP gate with 10 external learners before adding a second project.

The launch prompts are self-contained. Each agent should be able to start from a clean context with only the prompt and the files it names.

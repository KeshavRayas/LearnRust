# Coordination rules

These rules exist to keep parallel agents from stepping on each other. The integrator enforces them.

## Workspace and branches

- Product workspace: `rustlab/` inside the repository.
- Each thread gets its own T3 Code worktree. Do not share a checkout between threads.
- Branch names: `ws/foundation`, `ws/content-engine`, `ws/check-engine`, `ws/runner`, `ws/cli`, `ws/unit-converter`, `ws/e2e-ci`, `ws/explain`, `ws/docs`.
- Base branch: `main`.
- Rebase on `main` before requesting review and again before merge.

## File ownership

Only the listed owner edits a file. If you need a change outside your ownership, report it in your thread and let the integrator route it.

| Path | Owner |
| --- | --- |
| `rustlab/Cargo.toml` | Integrator |
| `rustlab/Cargo.lock` | Integrator |
| `rustlab/rust-toolchain.toml` | Integrator |
| `rustlab/.cargo/config.toml` | Integrator |
| `rustlab/crates/rustlab-core/**` | Integrator |
| `rustlab/crates/rustlab-content/**` | Content engine |
| `rustlab/crates/rustlab-check/**` | Check engine |
| `rustlab/crates/rustlab-runner/**` | Runner |
| `rustlab/crates/rustlab-cli/**` | CLI |
| `rustlab/crates/rustlab-explain/**` | Explain |
| `rustlab/content/errors.json` | Explain |
| `rustlab/content/tracks/foundations/track.toml` | Content author |
| `rustlab/content/tracks/foundations/projects/01-unit-converter/**` | Content author |
| `rustlab/xtask/**` | E2E/CI |
| `.github/workflows/rustlab-ci.yml` | E2E/CI |
| `rustlab/fixtures/**` | E2E/CI |
| `rustlab/README.md`, `rustlab/CONTRIBUTING.md` | Docs |
| `rustlab/docs/quickstart.md`, `rustlab/docs/architecture.md`, `rustlab/docs/content-authoring.md`, `rustlab/docs/troubleshooting.md`, `rustlab/docs/release.md` | Docs |
| `docs/build-plan/contracts.md` | Integrator |
| `docs/build-plan/status.md` | Integrator |

`docs/build-plan/README.md`, `coordination.md`, `prompts.md`, and the workstream briefs are planning artifacts. Agents read them and do not edit them.

## Contract changes

`rustlab/crates/rustlab-core/**` is frozen after WS0.

If an agent needs a new field, type, or trait method:

1. Stop work on the feature.
2. Write a short proposal in the thread: what is missing, which crate needs it, and the smallest change.
3. Open a PR titled `contract: <change>` that touches only `docs/build-plan/contracts.md` and `rustlab/crates/rustlab-core/**`.
4. The integrator reviews and merges it.
5. Rebase your branch and continue.

Do not add a parallel type in your own crate to avoid the contract process. That creates two sources of truth.

## Pull request rules

- One workstream per PR.
- Keep the diff under 400 changed lines where practical.
- Include tests for the new behavior.
- Run the acceptance commands in the workstream brief.
- Include a short summary, the test output, and any blockers.
- Do not commit generated files, `target/`, or editor settings.
- Do not reformat unrelated files.
- Do not edit `Cargo.lock` by hand. If a dependency change creates a conflict, ask the integrator to resolve it.

PR title format:

```text
<workstream>: <short description>
```

Examples:

```text
runner: add process-tree cleanup on Windows
content-engine: parse and validate project manifests
cli: implement doctor and start
```

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

The integrator merges in this order even if a later PR finishes first. The CLI is the integration point, so it merges after its dependencies. E2E merges after the CLI. Docs merge last.

## Integration cadence

- Merge to `main` at least once per day during Wave 1.
- No branch lives longer than three days without a rebase.
- The integrator updates `docs/build-plan/status.md` after every merge.
- A red `main` blocks all merges until it is fixed. The agent who broke it fixes it first.

## Status board

`docs/build-plan/status.md` tracks:

| Workstream | Branch | State | PR | Blocker |
| --- | --- | --- | --- | --- |
| Foundation | `ws/foundation` | not started | | |
| Content engine | `ws/content-engine` | not started | | |
| Check engine | `ws/check-engine` | not started | | |
| Runner | `ws/runner` | not started | | |
| CLI | `ws/cli` | not started | | |
| Unit converter | `ws/unit-converter` | not started | | |
| E2E/CI | `ws/e2e-ci` | not started | | |
| Explain | `ws/explain` | not started | | |
| Docs | `ws/docs` | not started | | |

States: `not started`, `in progress`, `blocked`, `review`, `merged`.

## Communication

- Each agent reports progress in its own thread.
- Blockers that affect another workstream go to the integrator, not directly to the other agent.
- The integrator is the only person who changes the merge order or the contracts.
- Decisions that change the plan are recorded in `docs/build-plan/status.md` or a new ADR in `docs/adr/`.

## Conflict resolution

If two agents need the same file:

1. Stop.
2. The integrator decides which workstream owns the change.
3. The other workstream waits for the owning PR to merge, then rebases.
4. Do not resolve the conflict by duplicating the code.

If a PR conflicts with `main`:

1. Rebase on `main`.
2. Re-run the acceptance commands.
3. If the conflict is in `Cargo.lock`, ask the integrator to resolve it.
4. If the conflict is in `rustlab-core`, the contract process applies.

## Definition of done for every PR

- The code compiles on Windows, macOS, and Linux.
- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- The workstream's acceptance tests pass.
- The PR description lists the commands used to verify.
- No files outside the ownership table are modified.

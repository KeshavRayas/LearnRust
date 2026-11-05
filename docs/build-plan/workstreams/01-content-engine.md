# WS1: Content engine

Agent: content
Branch: `ws/content-engine`
Wave: 1
Depends on: WS0
Merge order: 2

## Goal

Load, validate, and index the course content. The CLI should be able to ask "what projects exist, what is the next milestone, and where are its files" without knowing the directory layout.

## Owns

- `rustlab/crates/rustlab-content/**`
- `rustlab/crates/rustlab-content/tests/fixtures/**`

## Must not touch

- `rustlab/crates/rustlab-core/**`
- `rustlab/content/**` (the content author owns it)
- `rustlab/crates/rustlab-cli/**`
- `rustlab/xtask/**`

## Depends on

The frozen types in `rustlab-core` and the manifest schema in [../contracts.md](../contracts.md).

## Deliverables

- `ContentIndex::load(root: &Path) -> anyhow::Result<ContentIndex>`
- `impl ContentProvider for ContentIndex` with the methods from the contracts
- `tracks() -> &[Track]`
- `project(id: &str) -> Option<&Project>`
- `project_dir(id: &str) -> Option<&Path>` returning the directory that contains `project.toml`
- `milestone(project: &str, milestone: &str) -> Option<&Milestone>`
- `next_milestone(project: &str, completed: &BTreeSet<String>) -> Option<&Milestone>`
- `milestone_key(project, milestone) -> String` using `"project/milestone"`
- A `ContentError` type for validation failures with the file path and a human message.

## Validation rules

Reject the content if any of these fail:

1. A track or project ID is empty or duplicated.
2. A project has no milestones.
3. A milestone has no checks.
4. Two milestones in a project share an ID.
5. Two checks in a milestone share an ID.
6. A referenced lesson, task, hint, solution, or starter path does not exist.
7. A path is absolute or contains `..`.
8. A starter directory has no `Cargo.toml`.
9. A `cargo_test` or `cli` check names a package that is not in the starter workspace.
10. A `compile_fail` check has an empty `error_code`.
11. A `regex` matcher does not compile.
12. A `cargo_test` check with `test = Some(name)` has no `tests/hidden/<name>.rs` file.

Validation errors should list every problem, not just the first one. A content author should be able to fix the whole file in one pass.

## Implementation notes

- Use `toml::from_str` and `serde`.
- Read `track.toml` first, then each project directory listed in the track.
- The track lists project directory names. The project ID comes from `project.toml`. Index by ID, not by directory name.
- Do not walk the filesystem recursively looking for projects. The track file is the index.
- Use `std::fs::canonicalize` and a prefix check to enforce path containment.
- Resolve every manifest path to an absolute path during load, after the containment check. The CLI should not join paths itself.
- Parse the starter `Cargo.toml` to collect package names. Do not run Cargo.
- Keep the crate free of process execution and network access.
- Tests use `tests/fixtures/` directories created by the test, not the real content tree.

## Tests

- A valid fixture with one track, one project, two milestones.
- Missing lesson file.
- Duplicate milestone ID.
- Duplicate check ID.
- Path traversal with `../`.
- Starter without `Cargo.toml`.
- A `cli` check whose package is missing.
- A `cargo_test` check whose hidden test file is missing.
- An invalid regex matcher.
- `next_milestone` skips completed milestones and returns the first incomplete one.
- `next_milestone` returns `None` when everything is complete.

## Acceptance

From `rustlab/`:

```text
cargo test -p rustlab-content
cargo clippy -p rustlab-content --all-targets -- -D warnings
cargo fmt -p rustlab-content -- --check
```

## Deliverable

A PR titled `content-engine: load and validate project manifests`.

The PR description must include the test output and one example validation error message.

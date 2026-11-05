# Content authoring

A project is a normal Cargo package plus a manifest. The manifest schema is frozen in [../../docs/build-plan/contracts.md](../../docs/build-plan/contracts.md).

## Layout

```text
content/tracks/foundations/
  track.toml
  projects/01-unit-converter/
    project.toml
    README.md
    starter/
      Cargo.toml
      src/lib.rs
      src/main.rs
      tests/smoke.rs
    lessons/01-parse.md
    tasks/01-parse.md
    hints/01-parse-1.md
    tests/hidden/parses_km.rs
    solutions/01-parse/src/lib.rs
    broken/01-parse/src/lib.rs
```

## Track file

```toml
id = "foundations"
title = "Foundations"
projects = ["01-unit-converter"]
```

The list contains directory names. The project ID comes from `project.toml`.

## Project manifest

```toml
id = "unit-converter"
title = "Unit converter"
pitch = "Convert units from the command line."
hours = 2
concepts = ["enums", "Result", "match"]
starter = "starter"

[[milestone]]
id = "parse"
title = "Parse a quantity"
lesson = "lessons/01-parse.md"
task = "tasks/01-parse.md"
hints = ["hints/01-parse-1.md"]
solution = "solutions/01-parse"

[[milestone.checks]]
id = "parses-km"
kind = "cargo_test"
package = "unit-converter"
test = "parses_km"
```

## Rules

- The starter must compile and fail at least one check.
- Every solution must pass every check.
- Hidden tests live in `tests/hidden/<name>.rs` and are copied into the learner's `tests/` directory at check time.
- Solutions are full files, not patches. The validator copies the solution directory over the project.
- Broken variants are optional. If present, they must fail a check.
- Paths must stay inside the project directory.
- Do not assert exact compiler messages. Assert error codes and parsed output.

## Validate

```text
cargo xtask validate
```

The command prints one row per milestone with solution, starter, and broken results. It exits non-zero if any expectation fails.

## Add a project

1. Create the directory under the track.
2. Add `project.toml`.
3. Add the starter package.
4. Write the first failing test.
5. Write the lesson and task.
6. Add two hints.
7. Add the solution.
8. Add the hidden test.
9. Run `cargo xtask validate`.

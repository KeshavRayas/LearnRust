# WS5: Unit converter content

Agent: content author
Branch: `ws/unit-converter`
Wave: 1
Depends on: WS0 (manifest schema)
Merge order: 5

## Goal

Author the first guided project. It must be small enough to finish in one sitting, real enough to keep, and structured so every milestone has a failing test before it has a lesson.

## Owns

- `rustlab/content/tracks/foundations/track.toml`
- `rustlab/content/tracks/foundations/projects/01-unit-converter/**`

## Must not touch

- `rustlab/crates/**`
- `rustlab/xtask/**`
- `rustlab/content/errors.json`
- `rustlab/fixtures/**`

## Depends on

The manifest schema in [../contracts.md](../contracts.md). Do not invent new fields. If the schema is missing something, report it to the integrator.

## Layout

```text
rustlab/content/tracks/foundations/
  track.toml
  projects/01-unit-converter/
    project.toml
    README.md
    starter/
      Cargo.toml
      src/main.rs
      src/lib.rs
      tests/smoke.rs
    lessons/
      01-parse.md
      02-convert.md
      03-reject.md
      04-cli.md
    tasks/
      01-parse.md
      02-convert.md
      03-reject.md
      04-cli.md
    hints/
      01-parse-1.md
      01-parse-2.md
      ...
    tests/hidden/
      parses_km.rs
      converts_km_to_miles.rs
      rejects_unknown.rs
      cli_converts.rs
    solutions/
      01-parse/
      02-convert/
      03-reject/
      04-cli/
    broken/
      01-parse/
      03-reject/
```

`solutions/<milestone>/` mirrors the starter layout with full files. The validator copies them over the project. Do not use patch files; Windows path handling is not worth the risk.

`broken/<milestone>/` is a deliberately wrong version used by the negative E2E test.

## Track file

```toml
id = "foundations"
title = "Foundations"
projects = ["01-unit-converter"]
```

The track lists project directory names. The project ID comes from `project.toml`.

## Project file

Use the schema in [../contracts.md](../contracts.md). The package name is `unit-converter`. The starter must compile on day one.

## Milestones

### 1. Parse a quantity

Public API:

```rust
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

pub enum Unit {
    Kilometers,
    Miles,
    Celsius,
    Fahrenheit,
}

pub enum ParseError {
    Empty,
    InvalidNumber(String),
    UnknownUnit(String),
}

pub fn parse_quantity(input: &str) -> Result<Quantity, ParseError>;
```

Accept `"10 km"`, `"10km"`, `" 10 KM "`, and `"10.5 mi"`. The unit is case-insensitive. The number is `f64`.

Hidden test `parses_km` asserts the value and unit for those inputs.

Starter: the function returns `Err(ParseError::Empty)` so the crate compiles and the test fails cleanly.

### 2. Convert a quantity

```rust
pub enum ConvertError {
    Incompatible { from: Unit, to: Unit },
}

pub fn convert(q: Quantity, target: Unit) -> Result<Quantity, ConvertError>;
```

Conversions:

- Kilometers to miles: `value * 0.621371`
- Miles to kilometers: `value / 0.621371`
- Celsius to Fahrenheit: `value * 9.0 / 5.0 + 32.0`
- Fahrenheit to Celsius: `(value - 32.0) * 5.0 / 9.0`
- Same unit: return the input

Hidden test `converts_km_to_miles` uses an epsilon of `1e-4`.

Starter: `convert` returns `Err(ConvertError::Incompatible { from: q.unit, to: target })`.

### 3. Reject unknown units

`parse_quantity("10 furlongs")` returns `Err(ParseError::UnknownUnit("furlongs"))`.
`parse_quantity("abc km")` returns `Err(ParseError::InvalidNumber("abc"))`.
`parse_quantity("")` returns `Err(ParseError::Empty)`.

No panics. Hidden test `rejects_unknown` asserts the error variants and that a panic does not occur.

Starter: the parse function is still the milestone 1 stub, so this test fails or passes for the wrong reason. The content author must check that the starter fails at least one check in `xtask validate`, not necessarily this one.

### 4. CLI

```text
$ cargo run -- 10 km mi
6.21 mi

$ cargo run -- 100 c f
212.00 F

$ cargo run -- 10 furlongs mi
error: unknown unit: furlongs
```

Exit codes: 0 success, 2 bad input. Use `std::env::args`, not `clap`. This project teaches the standard library, not the ecosystem.

Hidden test `cli_converts` uses `std::process::Command` and `env!("CARGO_BIN_EXE_unit-converter")`. Do not add `assert_cmd` to the learner project.

Starter: `main` prints `not implemented` and exits 1.

## Lessons and tasks

Each lesson is 250 to 450 words. It explains one idea, shows the compiler error the learner will meet, and links to one Rust Book chapter. Each task states the goal, the API, the acceptance command, and one example.

Do not write a full tutorial. The failing test is the teacher.

## Hints

Two or three per milestone. Each hint is a question or a pointer, not a solution.

Bad: "Use `split_whitespace` and `parse::<f64>()`."
Good: "What happens if the input has no space between the number and the unit? Which method can split on either?"

## README

Include:

- The pitch: a unit converter you will actually use.
- A terminal demo.
- The four milestones and the concepts.
- How to run and test.
- What to do when stuck.

## Acceptance

From the project directory:

```text
cd rustlab/content/tracks/foundations/projects/01-unit-converter/starter
cargo test
```

The starter must compile and fail at least one test.

Copy each solution over the starter, copy the matching hidden test into `tests/`, and run:

```text
cargo test
```

The solution must pass all tests.

Also run:

```text
cargo fmt --check
cargo clippy -- -D warnings
```

## Deliverable

A PR titled `unit-converter: four guided milestones`.

The PR description must include the starter failure output and the solution pass output for all four milestones.

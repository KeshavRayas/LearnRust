# Learn Rust by shipping

Research and architecture for a project-first Rust learning tool.
November 2026.

## The short version

Build a local-first project ladder. A Rust CLI scaffolds real Cargo projects, gives every milestone a failing test, checks behavior in seconds, and keeps progress on disk. Content lives in TOML, Markdown, and Cargo workspaces, so the same material can later power a browser IDE without a rewrite.

Do not start with the browser. Start with the feedback loop.

The reference repo, [project-based-learning](https://github.com/practical-tutorials/project-based-learning), has 289k stars. Its Rust section has 11 top-level links, and most are from the 2014-2020 era. It is a menu of project ideas, not a sequence, and it has no tests, setup help, or Rust-specific teaching. That is the opening.

## What the market already does

| Tool | Model | What it gets right | Where it stops |
| --- | --- | --- | --- |
| Rustlings | Local CLI, about 90 exercises | Compiler-error drills, watch mode, zero ceremony | Single-file exercises, no multi-file projects |
| 100 Exercises to Learn Rust | Local workspace, 100 test-driven exercises | Careful concept order, real tests, solutions | Ends at exercises, not a portfolio; CC BY-NC content |
| Rustfinity | Browser, Docker-run tests, 171 challenges | Zero setup, tracks, instant feedback | Challenge-shaped, proprietary license |
| CodeCrafters | Hosted, staged real-world projects | Real systems projects with a test harness | Paid, assumes Rust fluency, little language teaching |
| Rust Playground | Hosted snippets | Instant compiler feedback | No curriculum, no progress, no project |
| Comprehensive Rust | Slides and exercises | Depth, classroom structure | A course, not a practice product |
| PBL repo | Curated links | Breadth and demand signal | No sequence, no verification, link rot |

The pattern is clear. The market has drills and it has advanced project challenges. It does not have a beginner-safe ladder between them.

## The gap

**A list is not a ladder.** A beginner cannot tell whether "write an OS" is the right next step after the borrow checker. The order matters more than the number of links.

**Exercises are not projects.** Rustlings teaches the conversation with the compiler. It does not teach module boundaries, error design, integration tests, or how to structure a 1,000-line program. Those are the skills that make someone independent.

**Projects are not scaffolded.** CodeCrafters and the PBL links assume the learner can bridge the gap. Beginners stall on setup, file layout, and "where does this function go," not only on ownership.

The product is the missing middle: a project ladder where each step has a runnable skeleton, a failing test, a short lesson, and a check that says what to fix next.

## Options

### Option A: Fork Rustlings and add a project mode

Rustlings is MIT, edition 2024, and has a proven loop. Forking is the fastest way to put something in front of learners.

Build effort: 1 to 2 engineer-weeks.

The problem is architecture. Rustlings is built around single-file exercises with `// TODO` markers and a `cargo run` or `cargo test` per exercise. Multi-file projects, hidden tests, and per-milestone progress are a different data model. You would inherit the TUI and the brand, then rewrite the core anyway.

Verdict: good for a two-week validation, weak as a foundation.

### Option B: Local-first CLI and content engine

A `rustlab` binary that manages real Cargo projects. The learner edits in VS Code, RustRover, Zed, or Neovim. The CLI runs checks, explains failures, and tracks progress.

Build effort: 4 to 6 engineer-weeks for the engine and three projects.

This is the recommended first build. It uses the real toolchain, supports real crates, runs offline, needs no sandbox, and leaves the learner with a portfolio of normal Cargo repositories. The feedback loop can be under five seconds with incremental builds.

The cost is setup. The learner needs Rust and a linker. On Windows, the missing MSVC build tools are the most common first failure, so a `rustlab doctor` command is part of the product, not a nice-to-have.

### Option C: Browser IDE with a hosted runner

A zero-install experience. Monaco editor, a Run button, server-side `cargo` in a container, accounts, and progress sync.

Build effort: 10 to 14 engineer-weeks, plus ongoing operations.

This is the long-term product if reach matters. It is also the most expensive and the riskiest. Arbitrary Rust code is hostile input. Docker alone is not a security boundary for public traffic. You need gVisor or Firecracker, no network, cgroup limits, a job queue, rate limits, and a plan for cold starts and compile cost. Crate support is another cliff: beginners will add `serde` in project three, and the sandbox must handle it.

Verdict: build it after the curriculum works, not before.

### Option D: Desktop app with a local toolchain

Tauri wraps the same web UI and runs checks against the learner's local Rust. No terminal, no sandbox, offline.

Build effort: 8 to 12 engineer-weeks.

This is a good phase-three wrapper around Option B. It is not a good first build. Cross-platform packaging, code signing, auto-update, and installer size will eat the time you should spend on content.

### Recommendation

Build B now. Keep the content schema and `CheckReport` contract identical to what C will need. If zero-install is a hard requirement from day one, build C first, but budget for the sandbox and treat the curriculum as a separate workstream.

Do not build "a prettier PBL list." The repo already wins that.

## The first hour

The product lives or dies in the first hour. This is the target experience.

```text
$ rustlab doctor
rustc 1.9x stable        ok
cargo                    ok
linker (MSVC)            ok
editor integration       VS Code, rust-analyzer found

$ rustlab start
Rustlab  Foundations

  [x] 01  Unit converter          2h
  [>] 02  Word count              3h   1/4 milestones
  [ ] 03  Todo CLI                4h
  [ ] 04  HTTP server             6h

$ rustlab check
✓ parses "10 km" as 10.0
✓ converts km to miles
✗ rejects an unknown unit
    expected: error "unknown unit: furlong"
    got:      panic at src/main.rs:42
    hint: return Err(...) instead of unwrap()

1 of 3 checks failed. Run `rustlab hint` for a nudge.
```

Seven rules make that experience work.

1. One command to start. `rustlab start` creates the project and opens the first task.
2. First green test in under ten minutes.
3. A failing test comes before the lesson. The learner sees the target before the theory.
4. A check finishes in under five seconds on an incremental build.
5. Three hints, then a solution diff. Being stuck forever is a design failure.
6. The project is a normal Cargo repository. It opens in any editor and can be pushed to GitHub.
7. No account, no cloud, no lock-in. Progress is a local JSON file.
8. Explain the error, not just the rule. The top 25 compiler errors get two plain sentences and one link.

## Curriculum

The ladder has seven projects. Projects 0 to 3 are guided: starter code, failing tests, explicit milestones. Projects 4 to 6 are open: a spec and a test harness, with the learner making design decisions.

| # | Project | Time | Rust you learn | What the learner ships |
| --- | --- | --- | --- | --- |
| 0 | Hello, cargo | 30m | Cargo, functions, `String`, tests | A repository that builds and tests |
| 1 | Unit converter CLI | 2h | Enums, `Result`, `match`, `FromStr`, `Display` | A tool they can actually use |
| 2 | `wc` and `grep` | 3h | Iterators, closures, `BufRead`, lifetimes, exit codes | Two Unix tools |
| 3 | Todo CLI with JSON | 4h | Structs, modules, `serde`, `?`, error enums | A daily-use task tracker |
| 4 | HTTP server from scratch | 6h | `TcpListener`, threads, `Arc<Mutex>`, HTTP, integration tests | A server they can `curl` |
| 5 | Redis-lite | 8h | Protocol parsing, `HashMap`, expiry, concurrency, error design | A server that speaks RESP |
| 6 | Choose your own | 8-20h | Ecosystem crates, ownership at scale, architecture | A portfolio piece |

Project 6 pulls from the PBL categories: text editor, CHIP-8 emulator, roguelike, WASM game, or a small interpreter. The learner picks, and the tool supplies a spec, a harness, and review checkpoints rather than step-by-step code.

Every project has the same parts:

- A pitch and a demo.
- A concept map, so the learner knows what the project is teaching.
- Three to eight milestones.
- A starter workspace that compiles on day one.
- Hidden tests that check behavior.
- Three levels of hints.
- A solution diff, available after an attempt.
- Extension ideas that reuse the PBL links.

Content is the long pole. A good project takes 20 to 40 hours to author, test, and write hints. Six projects is 150 to 250 hours. The software should make that authoring cheap, because the curriculum is the moat, not the runner.

## Architecture

Content is data. The runner is a service. The UI is a shell.

```text
┌──────────────────────────────────────────────────────────┐
│ Surfaces                                                 │
│  rustlab CLI/TUI      static web      desktop (Tauri)    │
└───────────────────────────┬──────────────────────────────┘
                            │ CheckReport (JSON)
┌───────────────────────────▼──────────────────────────────┐
│ Core                                                     │
│  domain model · progress · check engine · hints · errors │
└───────────────────────────┬──────────────────────────────┘
                            │ Runner trait
┌───────────────────────────▼──────────────────────────────┐
│ Runners                                                  │
│  LocalCargo · DockerSandbox · Mock                       │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│ Content                                                  │
│  tracks · projects · milestones · tests · solutions      │
│  TOML manifests · Markdown · starter Cargo workspaces    │
└──────────────────────────────────────────────────────────┘
```

Three decisions matter more than the rest.

**Content is Cargo, not a database.** Every project is a normal Cargo workspace. The learner can open it in any editor, commit it, and keep it. There is no proprietary exercise format to escape.

**Checks test behavior, not source text.** A check runs a test, spawns a binary, compares parsed output, or expects a compiler error code. It never greps for `let mut`. This teaches testing by example and makes cheating less interesting.

**Local first, hosted later.** The `Runner` trait has a local implementation and a container implementation. The CLI uses local. The web app uses the container. Both return the same `CheckReport`.

### Repository layout

```text
rustlab/
  Cargo.toml
  crates/
    rustlab-core/        domain types, no I/O
    rustlab-content/     manifest parsing, embedded content, validation
    rustlab-runner/      Runner trait, local cargo, docker sandbox
    rustlab-check/       check implementations and diagnostics
    rustlab-cli/         clap, ratatui, commands
    rustlab-server/      axum, phase 3
  content/
    tracks/
      foundations/
        track.toml
        projects/
          01-unit-converter/
            project.toml
            README.md
            starter/
            tests/
            hints/
            solutions/
  web/                   SvelteKit, phase 2
  xtask/                 validate, e2e, new-project, release
```

### Domain model

```rust
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub pitch: String,
    pub hours: u8,
    pub concepts: Vec<Concept>,
    pub milestones: Vec<Milestone>,
    pub starter: PathBuf,
}

pub struct Milestone {
    pub id: MilestoneId,
    pub title: String,
    pub lesson: PathBuf,
    pub task: PathBuf,
    pub checks: Vec<Check>,
    pub hints: Vec<PathBuf>,
    pub solution: PathBuf,
}

#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Check {
    CargoTest { package: String, test: Option<String> },
    Cli { package: String, args: Vec<String>, stdin: Option<String>, expect: Expect },
    CompileFail { package: String, error_code: String },
    Clippy { package: String, deny: Vec<String> },
}
```

`CheckReport` is the contract between every surface:

```rust
pub struct CheckReport {
    pub milestone: MilestoneId,
    pub passed: bool,
    pub results: Vec<CheckResult>,
    pub diagnostics: Vec<Diagnostic>,
    pub duration: Duration,
}
```

`Diagnostic` mirrors rustc: level, message, error code, spans, children, and suggestions. The CLI renders it as text. The web editor renders it as markers. One parser, two surfaces.

### Content format

```toml
id = "unit-converter"
title = "Unit converter"
hours = 2
concepts = ["enums", "Result", "match", "traits"]
edition = "2024"

[[milestone]]
id = "parse"
title = "Parse a quantity"
lesson = "lessons/parse.md"
task = "tasks/parse.md"
hints = ["hints/parse-1.md", "hints/parse-2.md"]
solution = "solutions/parse.patch"
checks = [
  { kind = "cargo_test", package = "unit-converter", test = "parses_km" },
  { kind = "cli", package = "unit-converter", args = ["10", "km", "mi"], expect = { stdout = "6.21" } },
]
```

The manifest is intentionally boring. Markdown holds the teaching. Cargo holds the code. The manifest only wires them together.

### Check engine

For library projects, run `cargo test -p <package> --test <test>`. For binary projects, build once and spawn the binary with args, stdin, and a timeout. Capture stdout, stderr, and exit code. Compare parsed values where possible, not exact formatting.

Hidden tests need care. The simplest local approach is to inject a gitignored `tests/_rustlab_hidden.rs`, run it, then remove it. This reuses the learner's `target/` directory, so checks stay fast. The hosted runner should copy the learner's source into a fresh workspace and add the hidden tests there, so the learner's tree is never modified.

Compile-fail lessons use `trybuild`. This is how ownership and lifetime errors get taught without "fix this line" puzzles. The check asserts an error code such as `E0382`, not the exact wording of the message.

The error explainer is a content feature with outsized value. Map the top 25 error codes to two plain sentences and one link to the Rust Book or Error Index. Store it as JSON. Use it in the CLI and the web app. Most beginners do not need a better compiler. They need a translation.

### Execution and sandboxing

The local runner is not a security boundary. The learner runs their own code. Still enforce a wall-clock timeout, an output cap, and a clean working directory. On Windows, kill the whole process tree with a job object. On Unix, use a process group.

The hosted runner is a security boundary. Minimum controls:

| Control | Setting |
| --- | --- |
| Network | none |
| User | non-root |
| Root filesystem | read-only |
| Temp filesystem | tmpfs, 64 MB |
| Memory | 512 MB |
| CPU | 1 core |
| PIDs | 64 |
| Wall clock | 15 s |
| Output | 64 KB |
| Secrets | none in the runner |
| Isolation | gVisor or Firecracker for public traffic |

Docker plus cgroups is a reasonable development setup. It is not enough for arbitrary public code. Cache registry downloads, never cache build artifacts from untrusted submissions. Rate-limit checks per user and keep a queue with backpressure.

### Web phase

SvelteKit for speed, Monaco for the editor. Axum for the API, Postgres for users and progress, a separate worker service for checks, and SSE for streaming output.

Do not start with rust-analyzer in the browser. The WASM build is experimental, large, and weak on crates and proc macros. For the first version, run `cargo check --message-format=json` on the server and render the diagnostics inline. Autocomplete is a later upgrade. The compiler is the teacher.

### Content pipeline

`cargo xtask validate` parses every manifest, checks IDs and links, and verifies two invariants: every solution passes every check, and every starter fails at least one check. Content cannot merge if either invariant breaks.

`cargo xtask e2e` is the repeatable artifact. It creates a temp directory, runs `rustlab start`, applies the solution patch, asserts `rustlab check` passes, applies a broken patch, asserts it fails, and verifies progress updates. Run it on Windows, macOS, and Linux in CI.

`cargo xtask new-project` scaffolds a project from a template. Authoring speed is a product feature.

## Roadmap

### Phase 1: prove the first hour, weeks 1-3

Build the CLI, the content schema, the local runner, `doctor`, and two projects. No web, no accounts, no AI.

Gate: 20 external learners. 60% finish project 1. Median time to first green under 15 minutes. If the gate fails, fix the first hour before adding content.

### Phase 2: prove the ladder, weeks 4-6

Add four more projects, the error explainer, hints, a static web viewer, and shareable progress. Add an optional Socratic hint that never emits a full solution.

Gate: 30% finish three projects. 25% return in week two. Check latency p95 under five seconds.

### Phase 3: prove the reach, weeks 7-12

Hosted runner, accounts, classroom dashboard, Tauri wrapper. The content and checks do not change.

Gate: 100 zero-install users. Under 5% of check failures caused by infrastructure.

### Metrics

- Activation: install to first check within 24 hours.
- Time to first green: median minutes.
- Project 1 completion.
- Milestone drop-off: the exact milestone where people quit.
- Hint rate and solution reveal rate.
- Week 2 return.
- Project 3 completion.
- Check latency p50 and p95.
- Infrastructure failure rate.

Kill criteria: if under 30% of starters finish project 1, the ladder or the check experience is wrong. Do not add projects. Fix the first hour.

## Risks

**Content quality.** A bad project teaches bad habits. Every solution must pass every check, and every project needs an expert review before release.

**Flaky checks.** Network, timing, and filesystem differences break trust. Keep checks hermetic. No network. No timing gates in beginner projects. Normalize output.

**Sandbox escape.** Public code execution is hostile. Use a real isolation layer, keep secrets out of the runner, and rate-limit.

**Scope creep.** Web, accounts, and AI can consume the schedule before the first project is good. The phase gates exist to prevent that.

**Rust churn.** Pin the toolchain with `rust-toolchain.toml`. Test stable and beta. Use edition 2024. Keep a one-command upgrade path for content.

**Windows.** The linker and process-tree kill are the two sharp edges. Test on Windows first, because that is where beginners fail silently.

## Licenses

- Rustlings: MIT. Forkable with attribution.
- 100 Exercises to Learn Rust: CC BY-NC 4.0. Do not reuse commercially.
- Rustfinity: proprietary. Do not reuse.
- Comprehensive Rust: Apache-2.0. Reusable with attribution.
- The Rust Book: MIT or Apache-2.0. Quote with attribution.
- PBL repo: MIT. The list is reusable, but the linked tutorials have their own licenses.

Borrow ideas, not files, unless the license allows it.

## Sources

- [project-based-learning](https://github.com/practical-tutorials/project-based-learning)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [100 Exercises to Learn Rust](https://github.com/mainmatter/100-exercises-to-learn-rust)
- [Rustfinity](https://www.rustfinity.com/)
- [CodeCrafters Rust track](https://codecrafters.io/rust)
- [Rust Playground](https://github.com/rust-lang/rust-playground)
- [Comprehensive Rust](https://github.com/google/comprehensive-rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

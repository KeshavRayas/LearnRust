# Frozen contracts

These interfaces are frozen after WS0. Parallel work depends on them. A change requires a contract-change PR approved by the integrator.

## Crate graph

```text
rustlab-core
  ^        ^          ^
  |        |          |
content  check     explain
  ^        ^
  |        |
  +----- runner
           ^
           |
         cli
           ^
           |
         xtask
```

Rules:

- `rustlab-core` has no dependency on any other workspace crate.
- `rustlab-content` depends on `rustlab-core`.
- `rustlab-check` depends on `rustlab-core`.
- `rustlab-runner` depends on `rustlab-core`.
- `rustlab-explain` depends on `rustlab-core`.
- `rustlab-cli` depends on all of the above.
- `xtask` depends on `rustlab-content`, `rustlab-check`, `rustlab-runner`, and `rustlab-core`.
- No cycles.

## Core types

```rust
// rustlab/crates/rustlab-core/src/lib.rs

pub type TrackId = String;
pub type ProjectId = String;
pub type MilestoneId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub title: String,
    pub projects: Vec<ProjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub pitch: String,
    pub hours: u8,
    pub concepts: Vec<String>,
    pub starter: PathBuf,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub title: String,
    pub lesson: PathBuf,
    pub task: PathBuf,
    pub hints: Vec<PathBuf>,
    pub solution: PathBuf,
    pub checks: Vec<CheckSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSpec {
    pub id: String,
    pub kind: CheckKind,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CheckKind {
    CargoTest {
        package: String,
        test: Option<String>,
    },
    Cli {
        package: String,
        args: Vec<String>,
        stdin: Option<String>,
        expect: Expect,
    },
    CompileFail {
        package: String,
        error_code: String,
    },
    Clippy {
        package: String,
        deny: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expect {
    #[serde(default)]
    pub exit_code: i32,
    #[serde(default)]
    pub stdout: Matcher,
    #[serde(default)]
    pub stderr: Matcher,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Matcher {
    #[default]
    Empty,
    Exact { value: String },
    Contains { value: String },
    Regex { value: String },
    Json { value: serde_json::Value },
}
```

Use struct variants for matchers. Internally tagged enums with newtype variants do not deserialize from TOML.

## Execution types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub stdin: Option<String>,
    pub env: BTreeMap<String, String>,
    pub timeout_secs: u64,
    pub output_cap_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
    pub duration_ms: u128,
}

pub trait ProcessRunner {
    fn run(&self, cmd: &CommandSpec) -> anyhow::Result<CommandOutput>;
}
```

## Check types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckContext {
    pub project_root: PathBuf,
    pub package_dir: PathBuf,
    pub target_dir: PathBuf,
    pub binary_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_id: String,
    pub passed: bool,
    pub message: String,
    pub detail: Option<String>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckReport {
    pub project: ProjectId,
    pub milestone: MilestoneId,
    pub passed: bool,
    pub results: Vec<CheckResult>,
    pub diagnostics: Vec<Diagnostic>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: Option<String>,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Diagnostic>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub file: PathBuf,
    pub line_start: usize,
    pub column_start: usize,
    pub line_end: usize,
    pub column_end: usize,
}
```

`rustlab-check` exposes:

```rust
pub fn plan(spec: &CheckSpec, ctx: &CheckContext) -> anyhow::Result<Vec<CommandSpec>>;
pub fn evaluate(spec: &CheckSpec, outputs: &[CommandOutput]) -> CheckResult;
pub fn parse_cargo_diagnostics(stdout: &str, stderr: &str) -> Vec<Diagnostic>;
```

`plan` does not spawn processes. `evaluate` does not spawn processes. The CLI orchestrates: plan, run each command, evaluate.

## Manifest schema

```toml
id = "unit-converter"
title = "Unit converter"
pitch = "Convert units from the command line."
hours = 2
concepts = ["enums", "Result", "match", "FromStr", "Display"]
edition = "2024"
starter = "starter"

[[milestone]]
id = "parse"
title = "Parse a quantity"
lesson = "lessons/01-parse.md"
task = "tasks/01-parse.md"
hints = ["hints/01-parse-1.md", "hints/01-parse-2.md"]
solution = "solutions/01-parse"

[[milestone.checks]]
id = "parses-km"
kind = "cargo_test"
package = "unit-converter"
test = "parses_km"

[[milestone.checks]]
id = "cli-km-to-mi"
kind = "cli"
package = "unit-converter"
args = ["10", "km", "mi"]
timeout_secs = 5
expect = { exit_code = 0, stdout = { kind = "contains", value = "6.21" } }
```

Paths are relative to the project directory. Reject absolute paths and `..` segments.

The content index resolves:

```rust
pub trait ContentProvider {
    fn tracks(&self) -> &[Track];
    fn project(&self, id: &str) -> Option<&Project>;
    fn project_dir(&self, id: &str) -> Option<&Path>;
    fn milestone(&self, project: &str, milestone: &str) -> Option<&Milestone>;
    fn next_milestone(&self, project: &str, completed: &BTreeSet<String>) -> Option<&Milestone>;
}

pub struct ContentIndex { /* ... */ }

impl ContentIndex {
    pub fn load(root: &Path) -> anyhow::Result<Self>;
}

impl ContentProvider for ContentIndex {
    /* ... */
}
```

`ContentProvider` lives in `rustlab-core`. `ContentIndex` lives in `rustlab-content`. The CLI depends on the trait, not the concrete index, so tests can use a fake content provider before WS1 lands.

## CLI surface

```text
rustlab doctor [--json]
rustlab list [--json]
rustlab start [<project>] [--dir <path>] [--force] [--json]
rustlab check [--json] [--project <id>] [--milestone <id>]
rustlab hint [--level <1|2|3>]
rustlab next [--json]
rustlab reset [--yes]
rustlab export [--output <path>]
```

Exit codes:

| Code | Meaning |
| --- | --- |
| 0 | Success |
| 1 | A check failed |
| 2 | Usage error |
| 3 | Environment or setup error |
| 4 | Content error |

`rustlab check --json` prints one `CheckReport` object to stdout. Human output goes to stderr when `--json` is set.

`rustlab start --json` prints:

```json
{
  "project": "unit-converter",
  "milestone": "parse",
  "path": "C:/work/unit-converter"
}
```

`rustlab list --json` prints:

```json
{
  "tracks": [
    {
      "id": "foundations",
      "title": "Foundations",
      "projects": [
        {
          "id": "unit-converter",
          "title": "Unit converter",
          "completed": 1,
          "total": 4
        }
      ]
    }
  ]
}
```

`rustlab doctor --json` prints:

```json
{
  "ok": true,
  "checks": [
    { "name": "rustc", "ok": true, "detail": "rustc 1.9x stable" },
    { "name": "cargo", "ok": true, "detail": "cargo 1.9x" },
    { "name": "linker", "ok": true, "detail": "link.exe found" }
  ]
}
```

## Project marker

`rustlab start` writes `<project>/.rustlab/project.json`:

```json
{
  "schema_version": 1,
  "project_id": "unit-converter",
  "milestone_id": "parse",
  "content_version": "0.1.0"
}
```

`rustlab check` uses the marker to find the project and milestone. If the marker is missing, it looks for a `project.toml` in the current directory and walks up.

## Progress

Default path: `RUSTLAB_HOME/state.json`.

`RUSTLAB_HOME` defaults to the platform data directory:

- Windows: `%LOCALAPPDATA%\rustlab`
- macOS: `~/Library/Application Support/rustlab`
- Linux: `~/.local/share/rustlab`

Schema:

```json
{
  "schema_version": 1,
  "learner_id": "6f1c2c4e-...",
  "completed": ["unit-converter/parse"],
  "current": "unit-converter/convert",
  "attempts": [
    {
      "project": "unit-converter",
      "milestone": "parse",
      "at": "2026-11-05T10:00:00Z",
      "passed": true
    }
  ]
}
```

Writes are atomic: write `state.json.tmp`, then rename. A corrupt file is moved to `state.json.bak` and replaced with a fresh state, with a warning.

## Environment variables

| Variable | Purpose |
| --- | --- |
| `RUSTLAB_HOME` | Override the state directory. Used by E2E. |
| `RUSTLAB_CONTENT` | Override the content directory. Used by E2E and development. |
| `RUSTLAB_LOG` | Log level, for example `info` or `debug`. |
| `NO_COLOR` | Disable ANSI color. |

## Change process

1. Open a PR titled `contract: <change>` that touches only `docs/build-plan/contracts.md` and `rustlab/crates/rustlab-core/**`.
2. The integrator reviews and merges it to `main`.
3. Every agent rebases on `main` before its next push.
4. No agent edits `rustlab-core` in a feature PR.

If an agent is blocked by a missing type, it reports the blocker in its thread instead of editing core.

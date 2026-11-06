# rustlab

Learn Rust by building real Cargo projects, starting from zero syntax.

`rustlab` gives you a project, a failing test, and a check command that runs the real compiler and real tests. It explains failures, gives hints, and keeps progress on disk.

## One command

```text
npx github:KeshavRayas/LearnRust
```

This installs the CLI, copies the course content to your local data directory, runs `rustlab doctor`, and creates the first project at `~/rustlab-projects/hello-rust`.

Requirements: Node 18+, Rust stable, and a linker. On Windows, install the Visual Studio C++ build tools.

For a non-interactive shell:

```text
npx --yes github:KeshavRayas/LearnRust
```

## First project

```text
cd ~/rustlab-projects/hello-rust
rustlab check
```

Open the file named in the task. When the check passes:

```text
rustlab next
```

## Commands

| Command | What it does |
| --- | --- |
| `rustlab doctor` | Checks rustc, cargo, the linker, content, and the progress directory |
| `rustlab list` | Lists projects and milestone progress |
| `rustlab start [project]` | Copies a starter project and writes the project marker |
| `rustlab check` | Runs the current milestone checks |
| `rustlab hint` | Prints the next hint |
| `rustlab solution` | Prints the worked solution after a failed attempt |
| `rustlab solution --write` | Applies the solution files |
| `rustlab next` | Advances after a passed milestone |
| `rustlab reset` | Restores the starter files |
| `rustlab export` | Prints the progress JSON |

Add `--json` to `doctor`, `list`, `start`, `check`, and `next` for machine-readable output.

## Projects

- **Hello, Rust**: first program, functions, `if`, vectors, loops, and slices.
- **Unit converter**: enums, `Result`, `match`, `Display`, and a real CLI.

## How it works

- Content lives in `content/`. Each project is a normal Cargo workspace with lessons, tasks, hidden tests, and solutions.
- Checks run the real `cargo` and the real binary. They test behavior, not source text.
- Progress lives in `RUSTLAB_HOME/state.json`. It defaults to the platform data directory.
- `cargo xtask validate` checks every solution, starter, and broken variant.
- `cargo xtask e2e` runs the learner path from `start` to `check`.

## Install from source

From the repository root:

```text
cd rustlab
cargo install --path crates/rustlab-cli
```

If you run from a clone, point the binary at the content:

PowerShell:

```powershell
$env:RUSTLAB_CONTENT = (Get-Location).Path + "\content"
```

macOS or Linux:

```bash
export RUSTLAB_CONTENT="$PWD/content"
```

## Docs

- [Quickstart](docs/quickstart.md)
- [Architecture](docs/architecture.md)
- [Content authoring](docs/content-authoring.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Release](docs/release.md)

## Status

MVP. Two projects, local-first. The web IDE and hosted runner are not part of this release.

## License

MIT. See the workspace `Cargo.toml` for the license field.

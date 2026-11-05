# rustlab

Learn Rust by building real Cargo projects.

`rustlab` scaffolds a project, gives every milestone a failing test, runs behavior checks, explains compiler errors, and keeps progress on disk. The first project is a unit converter with four milestones.

## Install from source

```text
cd rustlab
cargo install --path crates/rustlab-cli
```

The binary needs the content directory. For a local build, point it at this repository:

PowerShell:

```powershell
$env:RUSTLAB_CONTENT = (Get-Location).Path + "\content"
```

macOS or Linux:

```bash
export RUSTLAB_CONTENT="$PWD/content"
```

To avoid setting it in every shell, use `setx RUSTLAB_CONTENT "<path>"` on Windows, or add the export line to your shell profile.

Then run:

```text
rustlab doctor
```

## First project

```text
rustlab start unit-converter
cd unit-converter
rustlab check
```

Open the project in any editor. When a check fails, run `rustlab hint`. When all checks pass, run `rustlab next`.

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

## How it works

- Content lives in `content/`. Each project is a normal Cargo workspace with lessons, tasks, hidden tests, and solutions.
- Checks run the real `cargo` and the real binary. They test behavior, not source text.
- Progress lives in `RUSTLAB_HOME/state.json`. It defaults to the platform data directory.
- `cargo xtask validate` checks every solution, starter, and broken variant.
- `cargo xtask e2e` runs the learner path from `start` to `check`.

## Docs

- [Quickstart](docs/quickstart.md)
- [Architecture](docs/architecture.md)
- [Content authoring](docs/content-authoring.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Release](docs/release.md)

## Status

MVP. One project, four milestones, local-first. The web IDE and hosted runner are not part of this release.

## License

MIT. See the workspace `Cargo.toml` for the license field.

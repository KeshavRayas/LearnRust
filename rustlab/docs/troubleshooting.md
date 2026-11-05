# Troubleshooting

## `linker 'link.exe' not found` on Windows

Install the Visual Studio C++ build tools with the Desktop development with C++ workload. Open a new terminal and run:

```text
rustlab doctor
```

The doctor compiles and runs a hello world, so it catches a missing linker.

## `content directory not found`

`rustlab` looks for `RUSTLAB_CONTENT`, then `./content`, then `content` next to the binary. From the repository:

```powershell
$env:RUSTLAB_CONTENT = (Get-Location).Path + "\content"
```

macOS or Linux:

```bash
export RUSTLAB_CONTENT="$PWD/content"
```

## `cargo` is not recognized

Restart the terminal after installing Rust. Check:

```text
cargo --version
```

If it still fails, add `%USERPROFILE%\.cargo\bin` to `PATH`.

## A check times out

Run the check with JSON output to see the detail:

```text
rustlab check --json
```

A timed-out process is killed with its child processes. If a check is slow because of a cold build, run it again.

## Progress looks wrong

Progress lives at `RUSTLAB_HOME/state.json`. Print it:

```text
rustlab export
```

To reset progress, delete the file or set `RUSTLAB_HOME` to a fresh directory. The project marker lives at `<project>/.rustlab/project.json`.

## The project marker is missing

Run `rustlab start <project>` again, or run `rustlab check --project <id> --milestone <id>` from the project directory.

## Antivirus blocks `target/`

Exclude the project directory from real-time scanning. Rust builds create many short-lived files.

## Long paths on Windows

Enable long path support in Windows, or keep projects under a short path such as `C:\work\`.

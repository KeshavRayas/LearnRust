# Quickstart

This takes about 10 minutes.

## 1. Install Rust

Install from [rustup.rs](https://rustup.rs). On Windows, choose the MSVC toolchain and install the Visual Studio C++ build tools.

## 2. Install rustlab

From the repository root:

```text
cd rustlab
cargo install --path crates/rustlab-cli
```

Point the binary at the content directory.

PowerShell:

```powershell
$env:RUSTLAB_CONTENT = (Get-Location).Path + "\content"
```

macOS or Linux:

```bash
export RUSTLAB_CONTENT="$PWD/content"
```

## 3. Check the environment

```text
rustlab doctor
```

Expected:

```text
✓ rustc    rustc 1.9x
✓ cargo    cargo 1.9x
✓ linker   compiled and ran a hello world
✓ content  loaded ...\content
✓ home     ...\rustlab
```

If the linker check fails on Windows, install the Visual Studio C++ build tools and open a new terminal.

## 4. Start the project

```text
rustlab start unit-converter
```

This creates `unit-converter/` and prints the first task.

```text
cd unit-converter
```

Open the folder in VS Code, RustRover, Zed, or Neovim.

## 5. Run the first check

```text
rustlab check
```

The starter fails on purpose. Read the first failure, open `src/lib.rs`, and make `parse_quantity` return a `Quantity`.

Run the check again:

```text
rustlab check
```

When it passes:

```text
rustlab next
```

## 6. When you are stuck

```text
rustlab hint
rustlab hint --level 2
```

The hints are questions, not answers.

If the hints run out, print the worked solution:

```text
rustlab solution
```

Apply it to the project:

```text
rustlab solution --write
```

## 7. Finish the project

The four milestones are parse, convert, reject, and CLI. At the end you have a working command-line tool:

```text
cargo run -- 10 km mi
6.21 mi
```

## Progress

Progress is stored in `RUSTLAB_HOME/state.json`. Print it with:

```text
rustlab export
```

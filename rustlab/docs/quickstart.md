# Quickstart

This takes about 10 minutes.

## 1. Install Rust

Install from [rustup.rs](https://rustup.rs). On Windows, choose the MSVC toolchain and install the Visual Studio C++ build tools.

## 2. Run the setup command

From any terminal:

```text
npx github:KeshavRayas/LearnRust
```

This installs `rustlab`, copies the course content, checks your toolchain, and creates the first project at `~/rustlab-projects/hello-rust`.

If npm asks to install the package, accept it. For a non-interactive shell:

```text
npx --yes github:KeshavRayas/LearnRust
```

## 3. Start the first project

```text
cd ~/rustlab-projects/hello-rust
rustlab check
```

The starter fails on purpose. Open `src/main.rs` and change `TODO` to `Hello, Rust!`.

Run the check again:

```text
rustlab check
```

When it passes:

```text
rustlab next
```

## 4. When you are stuck

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

## 5. Finish the beginner project

Hello, Rust has four milestones:

1. Print a greeting.
2. Greet by name.
3. Count upward.
4. Add numbers.

Then continue with the unit converter:

```text
rustlab start unit-converter
```

## 6. Check the environment

```text
rustlab doctor
```

Expected:

```text
✓ rustc    rustc 1.9x
✓ cargo    cargo 1.9x
✓ linker   compiled and ran a hello world
✓ content  loaded ...\rustlab\content
✓ home     ...\rustlab
```

If the linker check fails on Windows, install the Visual Studio C++ build tools and open a new terminal.

## Progress

Progress is stored in `RUSTLAB_HOME/state.json`. Print it with:

```text
rustlab export
```

# LearnRust

Learn Rust by building real projects, starting from zero syntax.

## One command

```text
npx github:KeshavRayas/LearnRust
```

The command installs the `rustlab` CLI, copies the course content, checks your toolchain, and creates the first project at `~/rustlab-projects/hello-rust`.

Requirements:

- Node 18 or newer
- Rust stable. If Rust is missing, install it from [rustup.rs](https://rustup.rs). On Windows, install the Visual Studio C++ build tools too.

If npm asks to install the package, accept it. For a non-interactive shell, use:

```text
npx --yes github:KeshavRayas/LearnRust
```

## What you get

- **Hello, Rust**: a zero-syntax first project. Print a greeting, write a function, use a vector, and add numbers.
- **Unit converter**: parse input, convert units, return typed errors, and build a real CLI.
- A local progress file, hints, worked solutions, and a check command that runs the real compiler.

## The loop

```text
cd ~/rustlab-projects/hello-rust
rustlab check
```

Edit the file named in the task. When the check passes:

```text
rustlab next
```

When you are stuck:

```text
rustlab hint
rustlab hint --level 2
rustlab solution
```

## Repository layout

- `rustlab/` is the product: CLI, content engine, check engine, runner, and course content.
- `npm/` is the one-command bootstrap.
- `projects/` is a local learner workspace and is gitignored.
- `docs/` contains the product research and the parallel build plan.

## Product docs

- [rustlab README](rustlab/README.md)
- [Quickstart](rustlab/docs/quickstart.md)
- [Architecture](rustlab/docs/architecture.md)
- [Content authoring](rustlab/docs/content-authoring.md)
- [Troubleshooting](rustlab/docs/troubleshooting.md)

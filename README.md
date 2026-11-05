# LearnRust

A Cargo project with runnable examples and notes for learning Rust.

## rustlab

The project-based learning CLI lives in [`rustlab/`](rustlab/). It scaffolds real Cargo projects, checks behavior, explains compiler errors, and tracks progress.

Start with [`rustlab/README.md`](rustlab/README.md) or the [quickstart](rustlab/docs/quickstart.md).

## Learning notes

- Notes: [RUST_NOTES.md](RUST_NOTES.md)
- Lessons: `examples/` (one file per topic, heavily commented)
- Your original program: `src/main.rs`

```powershell
cargo run                          # your hello world
cargo run --example ownership      # one lesson
cargo check                        # fast type check while editing
cargo test                         # once you start writing tests
cargo clippy -- -D warnings        # lints
```

Start with `cargo run --example ownership`, then read section 5 of the notes,
then write your own version of the example from scratch.

If a command fails, read the top error first. If it says
`could not find Cargo.toml`, you are in the wrong directory. If it says
`linker 'link.exe' not found`, install the Visual Studio C++ build tools.

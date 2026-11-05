// Your original file, kept here as a standalone example.
//
// With Cargo:
//     cargo run --example hello_world
//
// Without Cargo (what you were doing before):
//     rustc hello_world.rs -o hello_world.exe
//     .\hello_world.exe
//
// `rustc` compiles one file and ignores everything else. Cargo is the build tool:
// dependencies, tests, examples, release builds. Use Cargo from now on.

fn main() {
    println!("Hello World");
}

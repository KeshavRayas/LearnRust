# Your first Rust program

A Rust program starts at `fn main()`. The `fn` keyword means "function". `main` is the function Rust runs first.

`println!` prints a line. The exclamation mark means it is a macro, not a normal function. For now, just remember the shape:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

The text inside the quotes is a string literal. The semicolon ends the statement.

Change the text in `src/main.rs`, then run `rustlab check`.

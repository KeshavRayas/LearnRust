# Functions and strings

A function takes input and returns output:

```rust
pub fn greet(name: &str) -> String {
    // ...
}
```

- `pub` means other files can call it.
- `name: &str` is a borrowed string. You can read it without taking ownership.
- `-> String` means the function returns an owned string.
- `String::new()` creates an empty owned string.
- `name.is_empty()` returns `true` when the name is empty.

Use `if` to choose between two results. The last expression in a function is the return value. Do not put a semicolon after it.

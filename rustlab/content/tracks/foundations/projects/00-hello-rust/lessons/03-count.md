# Vectors and loops

A `Vec<u32>` is a growable list of unsigned 32-bit integers.

```rust
let mut numbers = Vec::new();
for i in 1..=n {
    numbers.push(i);
}
```

- `let mut` makes the variable mutable. Without `mut`, you cannot push.
- `Vec::new()` creates an empty vector.
- `1..=n` is an inclusive range: 1, 2, 3, up to n.
- `push` adds an item to the end.

The function returns the vector as its last expression.

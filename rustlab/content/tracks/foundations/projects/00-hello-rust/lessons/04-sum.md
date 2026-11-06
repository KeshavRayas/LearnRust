# Slices and references

A slice is a view into a list:

```rust
pub fn sum(numbers: &[u32]) -> u32 {
    // ...
}
```

- `&[u32]` borrows a sequence of `u32` values. It does not own the list.
- A `for` loop over a slice gives you references to the items, so `number` is `&u32`.
- Use `*number` to read the value behind the reference.
- `let mut total = 0;` starts the accumulator.
- `total += *number;` adds each value.

Return `total` as the last expression.

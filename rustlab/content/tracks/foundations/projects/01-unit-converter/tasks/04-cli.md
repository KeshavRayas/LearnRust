# Task: wire up the CLI

Replace the `not implemented` body in `src/main.rs`.

Accept exactly three arguments: value, from unit, to unit.

```text
cargo run -- 10 km mi
```

prints:

```text
6.21 mi
```

Bad input prints an error to stderr and exits with code 2.

Run:

```text
rustlab check
```

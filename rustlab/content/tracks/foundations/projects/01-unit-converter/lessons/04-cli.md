# Wire up the CLI

`std::env::args()` gives you the command-line arguments. Skip the program name, collect the rest, and check the count.

Return `std::process::ExitCode` from `main`. Use `ExitCode::SUCCESS` for a good run and `ExitCode::from(2)` for bad input.

Print the result with two decimals:

```rust
println!("{:.2} {}", value, unit.symbol());
```

The hidden test runs the binary, so the output and the exit code are part of the contract.

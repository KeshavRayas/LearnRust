# Reject bad input

Good tools fail clearly. `"10 furlongs"` should return `Err(ParseError::UnknownUnit("furlongs"))`, not panic.

Keep the error type small:

- `Empty` for no input.
- `InvalidNumber` for text that is not a number.
- `UnknownUnit` for a unit you do not support.

Reject extra tokens too. `"10 km mi"` has two units, so it is not a valid quantity.

Implement `Display` for the error so the CLI can print it. Do not use `unwrap` on user input. `unwrap` is for cases where the program has already proven the value cannot fail.

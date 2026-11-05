Do not use `unwrap` on the result of `parse::<f64>()`. Use `map_err` to turn the parse error into `ParseError::InvalidNumber`.

The unknown unit error should keep the original unit text so the CLI can print it.

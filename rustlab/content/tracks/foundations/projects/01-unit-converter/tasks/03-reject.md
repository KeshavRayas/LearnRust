# Task: reject bad input

Make sure these return errors, not panics:

- `parse_quantity("")` returns `ParseError::Empty`
- `parse_quantity("abc km")` returns `ParseError::InvalidNumber("abc")`
- `parse_quantity("10 furlongs")` returns `ParseError::UnknownUnit("furlongs")`
- `parse_quantity("10 km mi")` returns an error for the extra token

Run:

```text
rustlab check
```

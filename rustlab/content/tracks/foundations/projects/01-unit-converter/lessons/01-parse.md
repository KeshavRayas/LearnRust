# Parse a quantity

The first step is turning text into a value. `"10 km"` should become a `Quantity { value: 10.0, unit: Unit::Kilometers }`.

Use `str::trim` and `str::split_whitespace` first. That handles `"10 km"` and `" 10 KM "`. Then handle the compact form `"10km"` by finding the first alphabetic character.

Return `Result<Quantity, ParseError>` instead of panicking. `ParseError` is an enum so the caller can tell an empty input from a bad number or an unknown unit.

The compiler will ask you to handle every `ParseError` variant when you match on it. That is the point.

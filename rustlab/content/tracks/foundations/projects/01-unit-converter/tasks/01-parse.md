# Task: parse a quantity

Implement `parse_quantity` in `src/lib.rs`.

It must accept:

- `"10 km"`
- `"10km"`
- `" 10 KM "`
- `"10.5 mi"`

It must return a `Quantity` with the parsed value and the matching `Unit`.

Run:

```text
rustlab check
```

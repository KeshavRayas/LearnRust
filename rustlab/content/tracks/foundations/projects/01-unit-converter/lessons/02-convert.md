# Convert a quantity

A conversion needs two units and one value. Match on the pair:

```rust
match (quantity.unit, target) {
    (Unit::Kilometers, Unit::Miles) => quantity.value * 0.621371,
    // ...
    _ => return Err(ConvertError::Incompatible { from: quantity.unit, to: target }),
}
```

The wildcard arm is important. It catches every pair you have not implemented, including length-to-temperature conversions.

If the units are the same, return the quantity unchanged. That is a valid conversion, not an error.

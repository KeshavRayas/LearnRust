Find the first character that is alphabetic. Everything before it is the number. Everything from it onward is the unit.

Then call `parse::<f64>()` on the number and map the error to `ParseError::InvalidNumber`.

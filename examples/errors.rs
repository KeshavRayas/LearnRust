// Errors: panic vs Result, the ? operator, custom error types.
//     cargo run --example errors
//
// Rule of thumb: Result for things that can reasonably fail (I/O, parsing,
// user input). panic! for bugs that should never happen (index out of bounds,
// broken invariant). A library should almost never panic on bad input.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Negative(i64),
}

// Display is what `{}` and `unwrap()`'s panic message use.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {e}"),
            AppError::Negative(n) => write!(f, "negative number not allowed: {n}"),
        }
    }
}

// Error lets `?` work with boxed trait objects and gives callers a `source()`.
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Parse(e) => Some(e),
            AppError::Negative(_) => None,
        }
    }
}

// From lets `?` convert automatically. This is the key ergonomics trick.
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i64 = s.parse()?; // ParseIntError -> AppError via From
    if n < 0 {
        return Err(AppError::Negative(n)); // early return
    }
    Ok(n as u32)
}

fn main() -> Result<(), AppError> {
    // Option: absence is normal, not an error.
    let maybe: Option<i32> = "12".parse().ok(); // .ok() turns Result into Option
    println!("maybe = {maybe:?}");

    // Three ways to handle a Result.
    match parse_positive("42") {
        Ok(n) => println!("match ok: {n}"),
        Err(e) => println!("match err: {e}"),
    }

    // Combinators: map, and_then, unwrap_or, unwrap_or_else.
    println!("map: {:?}", parse_positive("8").map(|n| n * 2));
    println!("unwrap_or: {}", parse_positive("bad").unwrap_or(0));

    // Errors implement Debug, so unwrap_err prints nicely.
    println!("bad input   -> {}", parse_positive("nope").unwrap_err());
    println!("negative    -> {}", parse_positive("-5").unwrap_err());

    // `?` also works in main when main returns Result.
    let n = parse_positive("7")?;
    println!("propagated from main: {n}");
    Ok(())
}

// Things that panic (don't do these with untrusted input):
//   vec![1][9]        index out of bounds
//   "x".parse::<i32>().unwrap()   on Err
//   x / 0             integer division by zero
//   .expect("...")    on None/Err
//
// For applications, the `anyhow` crate gives you anyhow::Result<T> and
// anyhow!("context") without writing custom error types.
// For libraries, the `thiserror` crate generates the Display/Error/From impls
// above from an enum with #[derive(Error)] and #[error("...")].

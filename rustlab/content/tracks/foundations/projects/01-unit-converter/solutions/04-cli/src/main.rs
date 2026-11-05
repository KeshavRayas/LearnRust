use std::process::ExitCode;

use unit_converter::{convert, parse_quantity, parse_unit};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("usage: unit-converter <value> <from> <to>");
        return ExitCode::from(2);
    }

    let input = format!("{} {}", args[0], args[1]);
    let quantity = match parse_quantity(&input) {
        Ok(quantity) => quantity,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };
    let target = match parse_unit(&args[2]) {
        Ok(unit) => unit,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };
    let converted = match convert(quantity, target) {
        Ok(quantity) => quantity,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };

    println!("{:.2} {}", converted.value, converted.unit.symbol());
    ExitCode::SUCCESS
}

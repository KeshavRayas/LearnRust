#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Kilometers,
    Miles,
    Celsius,
    Fahrenheit,
}

impl Unit {
    pub fn symbol(self) -> &'static str {
        match self {
            Unit::Kilometers => "km",
            Unit::Miles => "mi",
            Unit::Celsius => "C",
            Unit::Fahrenheit => "F",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Empty,
    InvalidNumber(String),
    UnknownUnit(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidNumber(value) => write!(f, "invalid number: {value}"),
            ParseError::UnknownUnit(unit) => write!(f, "unknown unit: {unit}"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ConvertError {
    Incompatible { from: Unit, to: Unit },
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Incompatible { from, to } => {
                write!(f, "cannot convert {} to {}", from.symbol(), to.symbol())
            }
        }
    }
}

impl std::error::Error for ConvertError {}

pub fn parse_unit(input: &str) -> Result<Unit, ParseError> {
    match input.trim().to_lowercase().as_str() {
        "km" => Ok(Unit::Kilometers),
        "mi" => Ok(Unit::Miles),
        "c" => Ok(Unit::Celsius),
        "f" => Ok(Unit::Fahrenheit),
        other => Err(ParseError::UnknownUnit(other.to_string())),
    }
}

pub fn parse_quantity(input: &str) -> Result<Quantity, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::Empty);
    }

    let mut parts = input.split_whitespace();
    let first = parts.next().ok_or(ParseError::Empty)?;
    let (number, unit) = match parts.next() {
        Some(unit) => (first.to_string(), unit.to_string()),
        None => {
            let split = input
                .find(|character: char| character.is_alphabetic())
                .ok_or_else(|| ParseError::InvalidNumber(input.to_string()))?;
            (input[..split].to_string(), input[split..].to_string())
        }
    };
    if parts.next().is_some() {
        return Err(ParseError::InvalidNumber(input.to_string()));
    }

    let value = number
        .trim()
        .parse::<f64>()
        .map_err(|_| ParseError::InvalidNumber(number.trim().to_string()))?;
    let unit = parse_unit(&unit)?;
    Ok(Quantity { value, unit })
}

pub fn convert(quantity: Quantity, target: Unit) -> Result<Quantity, ConvertError> {
    if quantity.unit == target {
        return Ok(quantity);
    }

    let value = match (quantity.unit, target) {
        (Unit::Kilometers, Unit::Miles) => quantity.value * 0.621371,
        (Unit::Miles, Unit::Kilometers) => quantity.value / 0.621371,
        (Unit::Celsius, Unit::Fahrenheit) => quantity.value * 9.0 / 5.0 + 32.0,
        (Unit::Fahrenheit, Unit::Celsius) => (quantity.value - 32.0) * 5.0 / 9.0,
        _ => {
            return Err(ConvertError::Incompatible {
                from: quantity.unit,
                to: target,
            })
        }
    };

    Ok(Quantity { value, unit: target })
}

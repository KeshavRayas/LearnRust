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

pub fn parse_unit(_input: &str) -> Result<Unit, ParseError> {
    Ok(Unit::Kilometers)
}

pub fn parse_quantity(_input: &str) -> Result<Quantity, ParseError> {
    Ok(Quantity {
        value: 0.0,
        unit: Unit::Kilometers,
    })
}

pub fn convert(quantity: Quantity, target: Unit) -> Result<Quantity, ConvertError> {
    Ok(Quantity {
        value: quantity.value,
        unit: target,
    })
}

use unit_converter::{parse_quantity, ParseError};

#[test]
fn rejects_unknown() {
    assert_eq!(
        parse_quantity("10 furlongs"),
        Err(ParseError::UnknownUnit("furlongs".to_string()))
    );
    assert_eq!(
        parse_quantity("abc km"),
        Err(ParseError::InvalidNumber("abc".to_string()))
    );
    assert_eq!(parse_quantity(""), Err(ParseError::Empty));
    assert!(parse_quantity("10 km mi").is_err());
}

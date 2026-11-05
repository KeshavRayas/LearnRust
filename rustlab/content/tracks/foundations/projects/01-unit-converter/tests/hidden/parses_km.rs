use unit_converter::{parse_quantity, Unit};

#[test]
fn parses_km() {
    let quantity = parse_quantity("10 km").expect("parse 10 km");
    assert!((quantity.value - 10.0).abs() < 1e-9);
    assert_eq!(quantity.unit, Unit::Kilometers);

    let quantity = parse_quantity("10km").expect("parse 10km");
    assert!((quantity.value - 10.0).abs() < 1e-9);
    assert_eq!(quantity.unit, Unit::Kilometers);

    let quantity = parse_quantity(" 10 KM ").expect("parse 10 KM");
    assert!((quantity.value - 10.0).abs() < 1e-9);
    assert_eq!(quantity.unit, Unit::Kilometers);
}

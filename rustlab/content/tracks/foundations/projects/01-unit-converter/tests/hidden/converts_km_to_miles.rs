use unit_converter::{convert, parse_quantity, Unit};

#[test]
fn converts_km_to_miles() {
    let quantity = parse_quantity("10 km").expect("parse 10 km");
    let miles = convert(quantity, Unit::Miles).expect("convert to miles");
    assert!((miles.value - 6.21371).abs() < 1e-4);
    assert_eq!(miles.unit, Unit::Miles);

    let celsius = parse_quantity("100 c").expect("parse 100 c");
    let fahrenheit = convert(celsius, Unit::Fahrenheit).expect("convert to fahrenheit");
    assert!((fahrenheit.value - 212.0).abs() < 1e-9);
    assert_eq!(fahrenheit.unit, Unit::Fahrenheit);
}

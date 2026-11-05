#[test]
fn crate_builds() {
    assert!(unit_converter::parse_quantity("").is_err());
}

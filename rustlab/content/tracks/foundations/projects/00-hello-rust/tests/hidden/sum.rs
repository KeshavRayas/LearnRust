use hello_rust::sum;

#[test]
fn sums_numbers() {
    assert_eq!(sum(&[1, 2, 3]), 6);
}

#[test]
fn sums_empty() {
    assert_eq!(sum(&[]), 0);
}

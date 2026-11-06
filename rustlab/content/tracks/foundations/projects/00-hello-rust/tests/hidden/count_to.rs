use hello_rust::count_to;

#[test]
fn counts_up() {
    assert_eq!(count_to(3), vec![1, 2, 3]);
}

#[test]
fn counts_zero() {
    assert!(count_to(0).is_empty());
}
